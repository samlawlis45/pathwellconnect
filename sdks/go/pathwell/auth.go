package pathwell

import (
	"crypto/hmac"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"crypto/x509"
	"encoding/base64"
	"encoding/pem"
	"fmt"
	"os"
	"time"
)

// KeyPair represents a public/private key pair
type KeyPair struct {
	PrivateKey string
	PublicKey  string
}

// GenerateKeyPair generates a new RSA key pair for agent authentication
func GenerateKeyPair() (*KeyPair, error) {
	privateKey, err := rsa.GenerateKey(rand.Reader, 2048)
	if err != nil {
		return nil, fmt.Errorf("failed to generate key pair: %w", err)
	}

	privateKeyPEM := pem.EncodeToMemory(&pem.Block{
		Type:  "RSA PRIVATE KEY",
		Bytes: x509.MarshalPKCS1PrivateKey(privateKey),
	})

	publicKeyDER, err := x509.MarshalPKIXPublicKey(&privateKey.PublicKey)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal public key: %w", err)
	}

	publicKeyPEM := pem.EncodeToMemory(&pem.Block{
		Type:  "PUBLIC KEY",
		Bytes: publicKeyDER,
	})

	return &KeyPair{
		PrivateKey: string(privateKeyPEM),
		PublicKey:  string(publicKeyPEM),
	}, nil
}

// LoadPrivateKey loads a private key from a file path
func LoadPrivateKey(keyPath string) (string, error) {
	data, err := os.ReadFile(keyPath)
	if err != nil {
		return "", fmt.Errorf("failed to read private key file: %w", err)
	}
	return string(data), nil
}

// SignRequest signs a request using the agent's private key
func SignRequest(
	privateKeyPEM string,
	method string,
	path string,
	body []byte,
	timestamp string,
) (string, error) {
	if timestamp == "" {
		timestamp = fmt.Sprintf("%d", time.Now().Unix())
	}

	// Create signature payload
	var bodyHash string
	if len(body) > 0 {
		hash := sha256.Sum256(body)
		bodyHash = fmt.Sprintf("%x", hash)
	}

	payload := fmt.Sprintf("%s\n%s\n%s\n%s", method, path, timestamp, bodyHash)

	// Parse private key
	block, _ := pem.Decode([]byte(privateKeyPEM))
	if block == nil {
		return "", fmt.Errorf("failed to decode PEM block")
	}

	privateKey, err := x509.ParsePKCS1PrivateKey(block.Bytes)
	if err != nil {
		return "", fmt.Errorf("failed to parse private key: %w", err)
	}

	// For MVP, use HMAC with private key material
	// In production, this would use proper cryptographic signing
	keyBytes := x509.MarshalPKCS1PrivateKey(privateKey)
	hmacKey := keyBytes[:32]

	mac := hmac.New(sha256.New, hmacKey)
	mac.Write([]byte(payload))
	signature := mac.Sum(nil)

	return base64.StdEncoding.EncodeToString(signature), nil
}

