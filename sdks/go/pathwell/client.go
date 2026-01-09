package pathwell

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"time"
)

// ClientOptions configures the Pathwell client
type ClientOptions struct {
	AgentID      string
	PrivateKeyPath string
	ProxyURL     string
	TargetURL    string
	HTTPClient   *http.Client
}

// Client is the main client for making authenticated requests through Pathwell proxy
type Client struct {
	agentID      string
	privateKey   string
	proxyURL     string
	targetURL    string
	httpClient   *http.Client
}

// NewClient creates a new Pathwell client
func NewClient(options ClientOptions) (*Client, error) {
	privateKey, err := LoadPrivateKey(options.PrivateKeyPath)
	if err != nil {
		return nil, fmt.Errorf("failed to load private key: %w", err)
	}

	proxyURL := options.ProxyURL
	if proxyURL == "" {
		proxyURL = "http://localhost:8080"
	}
	if proxyURL[len(proxyURL)-1] == '/' {
		proxyURL = proxyURL[:len(proxyURL)-1]
	}

	targetURL := options.TargetURL
	if targetURL == "" {
		targetURL = proxyURL
	}

	httpClient := options.HTTPClient
	if httpClient == nil {
		httpClient = &http.Client{
			Timeout: 30 * time.Second,
		}
	}

	return &Client{
		agentID:    options.AgentID,
		privateKey: privateKey,
		proxyURL:   proxyURL,
		targetURL:  targetURL,
		httpClient: httpClient,
	}, nil
}

// Call makes an authenticated request through Pathwell proxy
func (c *Client) Call(
	method string,
	requestURL string,
	headers map[string]string,
	body interface{},
) (*http.Response, error) {
	// Parse URL
	parsedURL, err := url.Parse(requestURL)
	if err != nil {
		return nil, fmt.Errorf("invalid URL: %w", err)
	}

	path := parsedURL.Path
	if parsedURL.RawQuery != "" {
		path += "?" + parsedURL.RawQuery
	}

	// Prepare body
	var bodyBytes []byte
	if body != nil {
		if bodyMap, ok := body.(map[string]interface{}); ok {
			bodyBytes, err = json.Marshal(bodyMap)
			if err != nil {
				return nil, fmt.Errorf("failed to marshal body: %w", err)
			}
		} else if bodyStr, ok := body.(string); ok {
			bodyBytes = []byte(bodyStr)
		} else if bodyBytesVal, ok := body.([]byte); ok {
			bodyBytes = bodyBytesVal
		} else {
			bodyBytes, err = json.Marshal(body)
			if err != nil {
				return nil, fmt.Errorf("failed to marshal body: %w", err)
			}
		}
	}

	// Prepare headers
	reqHeaders := make(map[string]string)
	for k, v := range headers {
		reqHeaders[k] = v
	}
	reqHeaders["X-Pathwell-Agent-ID"] = c.agentID

	// Sign request
	timestamp := fmt.Sprintf("%d", time.Now().Unix())
	signature, err := SignRequest(c.privateKey, method, path, bodyBytes, timestamp)
	if err != nil {
		return nil, fmt.Errorf("failed to sign request: %w", err)
	}
	reqHeaders["X-Pathwell-Signature"] = signature
	reqHeaders["X-Pathwell-Timestamp"] = timestamp

	// Build proxy URL
	proxyURL := c.proxyURL + path

	// Create request
	req, err := http.NewRequest(method, proxyURL, bytes.NewReader(bodyBytes))
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}

	// Set headers
	for k, v := range reqHeaders {
		req.Header.Set(k, v)
	}

	// Make request
	return c.httpClient.Do(req)
}

// Get makes a GET request
func (c *Client) Get(url string, headers map[string]string) (*http.Response, error) {
	return c.Call("GET", url, headers, nil)
}

// Post makes a POST request
func (c *Client) Post(url string, headers map[string]string, body interface{}) (*http.Response, error) {
	return c.Call("POST", url, headers, body)
}

// Put makes a PUT request
func (c *Client) Put(url string, headers map[string]string, body interface{}) (*http.Response, error) {
	return c.Call("PUT", url, headers, body)
}

// Patch makes a PATCH request
func (c *Client) Patch(url string, headers map[string]string, body interface{}) (*http.Response, error) {
	return c.Call("PATCH", url, headers, body)
}

// Delete makes a DELETE request
func (c *Client) Delete(url string, headers map[string]string) (*http.Response, error) {
	return c.Call("DELETE", url, headers, nil)
}

