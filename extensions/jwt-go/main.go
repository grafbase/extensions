package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
	"strings"
	"time"
)

// Configuration represents the extension configuration
type Configuration struct {
	URL              string        `json:"url"`
	Issuer           string        `json:"issuer,omitempty"`
	Audience         string        `json:"audience,omitempty"`
	PollInterval     time.Duration `json:"poll_interval,omitempty"`
	HeaderName       string        `json:"header_name,omitempty"`
	HeaderValuePrefix string       `json:"header_value_prefix,omitempty"`
	CookieName       string        `json:"cookie_name,omitempty"`
	Locations        []Location    `json:"locations,omitempty"`
}

// Location represents a place to look for the JWT token
type Location struct {
	Type         string `json:"type"`
	Name         string `json:"name"`
	ValuePrefix  string `json:"value_prefix,omitempty"`
}

// JWTExtension represents the main extension struct
type JWTExtension struct {
	Config  Configuration
	Decoder *Decoder
}

// Initialize sets up the extension with the provided configuration
func (j *JWTExtension) Initialize(configJSON []byte) error {
	// Parse configuration
	var configMap map[string]interface{}
	if err := json.Unmarshal(configJSON, &configMap); err != nil {
		return fmt.Errorf("failed to parse configuration: %w", err)
	}

	// Set default poll interval if not specified
	pollInterval := time.Minute
	if val, ok := configMap["poll_interval"].(float64); ok {
		pollInterval = time.Duration(val) * time.Second
	}

	// Parse URL
	rawURL, ok := configMap["url"].(string)
	if !ok {
		return fmt.Errorf("url is required and must be a string")
	}

	jwksURL, err := url.Parse(rawURL)
	if err != nil {
		return fmt.Errorf("invalid url: %w", err)
	}

	// Create config
	config := Configuration{
		URL:          rawURL,
		PollInterval: pollInterval,
	}

	// Set optional fields
	if issuer, ok := configMap["issuer"].(string); ok {
		config.Issuer = issuer
	}

	if audience, ok := configMap["audience"].(string); ok {
		config.Audience = audience
	}

	if headerName, ok := configMap["header_name"].(string); ok {
		config.HeaderName = headerName
	}

	if headerValuePrefix, ok := configMap["header_value_prefix"].(string); ok {
		config.HeaderValuePrefix = headerValuePrefix
	}

	if cookieName, ok := configMap["cookie_name"].(string); ok {
		config.CookieName = cookieName
	}

	// Set up locations
	locations := []Location{}

	// Add header location if specified
	if config.HeaderName != "" {
		locations = append(locations, Location{
			Type:        "header",
			Name:        config.HeaderName,
			ValuePrefix: config.HeaderValuePrefix,
		})
	}

	// Add cookie location if specified
	if config.CookieName != "" {
		locations = append(locations, Location{
			Type: "cookie",
			Name: config.CookieName,
		})
	}

	// If no locations specified, use default header location
	if len(locations) == 0 {
		locations = append(locations, Location{
			Type:        "header",
			Name:        "Authorization",
			ValuePrefix: "Bearer ",
		})
	}

	config.Locations = locations

	// Create decoder
	decoder, err := NewDecoder(jwksURL, config.PollInterval, config.Issuer, config.Audience)
	if err != nil {
		return fmt.Errorf("failed to create decoder: %w", err)
	}

	j.Config = config
	j.Decoder = decoder

	return nil
}

// Authenticate implements the authentication logic for the extension
func (j *JWTExtension) Authenticate(headers http.Header) ([]byte, error) {
	// Try to find token from the configured locations
	var token string
	var found bool

	for _, location := range j.Config.Locations {
		switch location.Type {
		case "header":
			if headerVal := headers.Get(location.Name); headerVal != "" {
				if location.ValuePrefix != "" {
					if strings.HasPrefix(headerVal, location.ValuePrefix) {
						token = strings.TrimPrefix(headerVal, location.ValuePrefix)
						found = true
					}
				} else {
					token = headerVal
					found = true
				}
			}

		case "cookie":
			if cookieHeader := headers.Get("Cookie"); cookieHeader != "" {
				cookies := strings.Split(cookieHeader, "; ")
				for _, cookie := range cookies {
					parts := strings.SplitN(cookie, "=", 2)
					if len(parts) == 2 && parts[0] == location.Name {
						token = parts[1]
						found = true
						break
					}
				}
			}
		}

		if found {
			break
		}
	}

	if !found || token == "" {
		return nil, fmt.Errorf("unauthorized: no token found")
	}

	// Decode and validate token
	payload, err := j.Decoder.Decode(token)
	if err != nil {
		return nil, fmt.Errorf("unauthorized: %w", err)
	}

	return payload, nil
}

func main() {}