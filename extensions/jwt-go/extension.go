package main

import (
	"fmt"
	"net/http"
	"os"
)

// grafbase_extension_version will be set at build time
var grafbase_extension_version string = "dev"

// The ExtensionContext represents the interface provided by Grafbase
type ExtensionContext interface {
	Logger() Logger
	Config() []byte
}

// Logger interface for the extension
type Logger interface {
	Debug(msg string)
	Info(msg string)
	Warn(msg string)
	Error(msg string)
}

// GrafbaseExtension is the main extension type that needs to be exported
type GrafbaseExtension struct {
	ctx     ExtensionContext
	extImpl *JWTExtension
}

// Initialize the extension
func Initialize(ctx ExtensionContext) *GrafbaseExtension {
	logger := ctx.Logger()
	logger.Info(fmt.Sprintf("Initializing jwt extension version %s", grafbase_extension_version))

	ext := &GrafbaseExtension{
		ctx:     ctx,
		extImpl: &JWTExtension{},
	}

	err := ext.extImpl.Initialize(ctx.Config())
	if err != nil {
		logger.Error(fmt.Sprintf("Failed to initialize extension: %v", err))
		os.Exit(1)
	}

	return ext
}

// Authenticate implements the authentication hook
func (e *GrafbaseExtension) Authenticate(headers http.Header) ([]byte, error) {
	logger := e.ctx.Logger()
	logger.Debug("Processing authentication request")

	payload, err := e.extImpl.Authenticate(headers)
	if err != nil {
		logger.Debug(fmt.Sprintf("Authentication failed: %v", err))
		return nil, err
	}

	return payload, nil
}
