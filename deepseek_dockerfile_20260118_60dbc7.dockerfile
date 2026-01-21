FROM nginx:alpine

# Copy mock responses
COPY health.json /usr/share/nginx/html/
COPY api/v1/chunks /usr/share/nginx/html/api/v1/chunks/

# Simple nginx config
RUN echo 'server { \
    listen 80; \
    location / { \
        root /usr/share/nginx/html; \
        try_files $uri $uri.json $uri/ =404; \
        add_header Content-Type application/json; \
    } \
}' > /etc/nginx/conf.d/default.conf

EXPOSE 80