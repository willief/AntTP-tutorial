#!/usr/bin/env python3
import http.server
import json
import socketserver

PORT = 18888

class AntTPHandler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/health':
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            response = json.dumps({"status": "healthy", "service": "AntTP Mock"})
            self.wfile.write(response.encode())
        elif self.path == '/api/v1/chunks':
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            response = json.dumps({"chunks": [], "total": 0})
            self.wfile.write(response.encode())
        else:
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            response = json.dumps({"message": "AntTP Mock API", "docs": "Try /health"})
            self.wfile.write(response.encode())
    
    def do_POST(self):
        content_length = int(self.headers['Content-Length'])
        post_data = self.rfile.read(content_length)
        
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        response = json.dumps({
            "status": "created",
            "address": "mock-address-" + str(hash(post_data))[-8:],
            "size": len(post_data)
        })
        self.wfile.write(response.encode())
    
    def log_message(self, format, *args):
        # Suppress the default logging
        pass

if __name__ == '__main__':
    with socketserver.TCPServer(("", PORT), AntTPHandler) as httpd:
        print(f"AntTP Mock Server running on port {PORT}")
        print(f"Try: curl http://localhost:{PORT}/health")
        httpd.serve_forever()
