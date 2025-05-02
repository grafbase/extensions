package main

import (
	"context"
	"encoding/json"
	"fmt"
	"net/url"
	"sync"
	"time"

	"github.com/lestrrat-go/jwx/v2/jwk"
	"github.com/lestrrat-go/jwx/v2/jws"
	"github.com/lestrrat-go/jwx/v2/jwt"
)

// Decoder handles JWT decoding and validation
type Decoder struct {
	jwksURL      *url.URL
	pollInterval time.Duration
	issuer       string
	audience     string
	keySet       jwk.Set
	lastFetch    time.Time
	mutex        sync.RWMutex
}

// NewDecoder creates a new JWT decoder with the specified JWKS URL
func NewDecoder(jwksURL *url.URL, pollInterval time.Duration, issuer, audience string) (*Decoder, error) {
	decoder := &Decoder{
		jwksURL:      jwksURL,
		pollInterval: pollInterval,
		issuer:       issuer,
		audience:     audience,
	}

	// Fetch initial key set
	if err := decoder.refreshKeys(); err != nil {
		return nil, fmt.Errorf("failed to fetch initial JWKS: %w", err)
	}

	return decoder, nil
}

// refreshKeys updates the JWKS from the URL if needed
func (d *Decoder) refreshKeys() error {
	d.mutex.Lock()
	defer d.mutex.Unlock()

	// Check if we need to refresh the keys based on poll interval
	if !d.lastFetch.IsZero() && time.Since(d.lastFetch) < d.pollInterval {
		return nil
	}

	// Fetch JWKS from URL
	set, err := jwk.Fetch(context.Background(), d.jwksURL.String())
	if err != nil {
		return fmt.Errorf("failed to fetch JWKS: %w", err)
	}

	d.keySet = set
	d.lastFetch = time.Now()

	return nil
}

// Decode validates and decodes a JWT token string, returning the token payload
func (d *Decoder) Decode(tokenString string) ([]byte, error) {
	// Refresh keys if needed
	if err := d.refreshKeys(); err != nil {
		return nil, fmt.Errorf("error refreshing keys: %w", err)
	}

	d.mutex.RLock()
	deferredKeySet := d.keySet
	d.mutex.RUnlock()

	// Parse token without verification to get header for kid
	_, err := jws.Parse([]byte(tokenString))
	if err != nil {
		return nil, fmt.Errorf("failed to parse token: %w", err)
	}

	// Parse and validate the token
	parseOptions := []jwt.ParseOption{
		jwt.WithKeySet(deferredKeySet),
		jwt.WithValidate(true),
		jwt.WithVerify(true),
	}

	// Add issuer validation if configured
	if d.issuer != "" {
		parseOptions = append(parseOptions, jwt.WithIssuer(d.issuer))
	}

	// Add audience validation if configured
	if d.audience != "" {
		parseOptions = append(parseOptions, jwt.WithAudience(d.audience))
	}

	// Parse and validate the token
	token, err := jwt.ParseString(tokenString, parseOptions...)
	if err != nil {
		return nil, fmt.Errorf("token validation failed: %w", err)
	}

	// Extract and return the token payload
	payload, err := json.Marshal(token.PrivateClaims())
	if err != nil {
		return nil, fmt.Errorf("failed to serialize token payload: %w", err)
	}

	return payload, nil
}
