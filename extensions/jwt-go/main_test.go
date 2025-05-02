package main

import (
	"encoding/json"
	"net/http"
	"testing"
	"time"
)

// MockExtensionContext implements the ExtensionContext interface for testing
type MockExtensionContext struct {
	config []byte
	logger MockLogger
}

func (m *MockExtensionContext) Logger() Logger {
	return &m.logger
}

func (m *MockExtensionContext) Config() []byte {
	return m.config
}

// MockLogger implements the Logger interface for testing
type MockLogger struct {
	logs []string
}

func (l *MockLogger) Debug(msg string) { l.logs = append(l.logs, "DEBUG: "+msg) }
func (l *MockLogger) Info(msg string)  { l.logs = append(l.logs, "INFO: "+msg) }
func (l *MockLogger) Warn(msg string)  { l.logs = append(l.logs, "WARN: "+msg) }
func (l *MockLogger) Error(msg string) { l.logs = append(l.logs, "ERROR: "+msg) }

// TestConfigParsing tests that the extension properly parses configuration
func TestConfigParsing(t *testing.T) {
	testConfig := `{
		"url": "https://example.com/.well-known/jwks.json",
		"issuer": "example.com",
		"audience": "my-api",
		"poll_interval": 120,
		"header_name": "X-Authorization",
		"header_value_prefix": "Token "
	}`

	ext := &JWTExtension{}
	err := ext.Initialize([]byte(testConfig))
	if err != nil {
		t.Fatalf("Failed to initialize extension: %v", err)
	}

	// Verify config was parsed correctly
	if ext.Config.URL != "https://example.com/.well-known/jwks.json" {
		t.Errorf("Expected URL %s, got %s", "https://example.com/.well-known/jwks.json", ext.Config.URL)
	}

	if ext.Config.Issuer != "example.com" {
		t.Errorf("Expected issuer %s, got %s", "example.com", ext.Config.Issuer)
	}

	if ext.Config.Audience != "my-api" {
		t.Errorf("Expected audience %s, got %s", "my-api", ext.Config.Audience)
	}

	expectedDuration := 120 * time.Second
	if ext.Config.PollInterval != expectedDuration {
		t.Errorf("Expected poll interval %v, got %v", expectedDuration, ext.Config.PollInterval)
	}

	if ext.Config.HeaderName != "X-Authorization" {
		t.Errorf("Expected header name %s, got %s", "X-Authorization", ext.Config.HeaderName)
	}

	if ext.Config.HeaderValuePrefix != "Token " {
		t.Errorf("Expected header value prefix %s, got %s", "Token ", ext.Config.HeaderValuePrefix)
	}

	// Verify locations were set up correctly
	if len(ext.Config.Locations) != 1 {
		t.Fatalf("Expected 1 location, got %d", len(ext.Config.Locations))
	}

	loc := ext.Config.Locations[0]
	if loc.Type != "header" {
		t.Errorf("Expected location type %s, got %s", "header", loc.Type)
	}

	if loc.Name != "X-Authorization" {
		t.Errorf("Expected location name %s, got %s", "X-Authorization", loc.Name)
	}

	if loc.ValuePrefix != "Token " {
		t.Errorf("Expected location value prefix %s, got %s", "Token ", loc.ValuePrefix)
	}
}

// TestDefaultConfig tests that default values are applied when not specified
func TestDefaultConfig(t *testing.T) {
	testConfig := `{
		"url": "https://example.com/.well-known/jwks.json"
	}`

	ext := &JWTExtension{}
	err := ext.Initialize([]byte(testConfig))
	if err != nil {
		t.Fatalf("Failed to initialize extension: %v", err)
	}

	// Verify default poll interval was set
	expectedDuration := 60 * time.Second
	if ext.Config.PollInterval != expectedDuration {
		t.Errorf("Expected default poll interval %v, got %v", expectedDuration, ext.Config.PollInterval)
	}

	// Verify default location was set
	if len(ext.Config.Locations) != 1 {
		t.Fatalf("Expected 1 default location, got %d", len(ext.Config.Locations))
	}

	loc := ext.Config.Locations[0]
	if loc.Type != "header" {
		t.Errorf("Expected default location type %s, got %s", "header", loc.Type)
	}

	if loc.Name != "Authorization" {
		t.Errorf("Expected default location name %s, got %s", "Authorization", loc.Name)
	}

	if loc.ValuePrefix != "Bearer " {
		t.Errorf("Expected default location value prefix %s, got %s", "Bearer ", loc.ValuePrefix)
	}
}

// TestTokenExtraction tests extraction of tokens from various locations
func TestTokenExtraction(t *testing.T) {
	// This test assumes decoder functionality is mocked or stubbed
	// Here we focus on testing the token extraction logic

	tests := []struct {
		name     string
		config   string
		headers  http.Header
		hasError bool
		expected string
	}{
		{
			name:   "Extract from Authorization header with Bearer prefix",
			config: `{"url": "https://example.com/.well-known/jwks.json"}`,
			headers: http.Header{
				"Authorization": []string{"Bearer token123"},
			},
			hasError: false,
			expected: "token123",
		},
		{
			name:   "Extract from custom header",
			config: `{"url": "https://example.com/.well-known/jwks.json", "header_name": "X-Token", "header_value_prefix": "JWT "}`,
			headers: http.Header{
				"X-Token": []string{"JWT token456"},
			},
			hasError: false,
			expected: "token456",
		},
		{
			name:   "Extract from cookie",
			config: `{"url": "https://example.com/.well-known/jwks.json", "cookie_name": "auth-token"}`,
			headers: http.Header{
				"Cookie": []string{"session=123; auth-token=token789; other=value"},
			},
			hasError: false,
			expected: "token789",
		},
		{
			name:     "No token found",
			config:   `{"url": "https://example.com/.well-known/jwks.json"}`,
			headers:  http.Header{},
			hasError: true,
			expected: "",
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			// Create extension with a mock decoder that just returns the token
			ext := &JWTExtension{}
			err := ext.Initialize([]byte(tc.config))
			if err != nil {
				t.Fatalf("Failed to initialize extension: %v", err)
			}

			// Override the decoder with a mock that just returns the token
			ext.Decoder = &mockDecoder{}

			result, err := ext.Authenticate(tc.headers)
			if tc.hasError && err == nil {
				t.Errorf("Expected an error but got none")
			}

			if !tc.hasError {
				if err != nil {
					t.Errorf("Did not expect an error but got: %v", err)
				}

				var extractedToken string
				json.Unmarshal(result, &extractedToken)
				if extractedToken != tc.expected {
					t.Errorf("Expected token %s, got %s", tc.expected, extractedToken)
				}
			}
		})
	}
}

// mockDecoder is a simplified decoder for testing
type mockDecoder struct{}

func (md *mockDecoder) Decode(tokenString string) ([]byte, error) {
	// Simply return the token string as the payload for testing
	return json.Marshal(tokenString)
}
