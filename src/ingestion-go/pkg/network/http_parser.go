package network

type HTTPParser struct{}

func NewHTTPParser() *HTTPParser {
	return &HTTPParser{}
}

func (p *HTTPParser) ParseRequest(payload []byte) error {
	return nil
}
