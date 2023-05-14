# rot 13 encryption as a microservice
I know this is not useful but I did it anyway

## Installation

Just run this image with docker: ``docker run -p 80:80 ghcr.io/adridevelopsthings/rot-13-aam`` or build it yourself with ``cargo build``. You can change the listen address by setting the enviroment variable ``LISTEN_ADDRESS``.

## Use the microservice
Just send HTTP GET requests to ``/Your Message`` or ``/your_cipher``, just like that:

```
GET /Hello%20World HTTP/1.1
``` 

The server response will look like this:
```
HTTP/1.1 200 OK

Uryyb Jbeyq
```

