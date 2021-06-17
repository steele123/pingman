# pingman
Pingman is a very fast pinger with an extremely verbose cli interface with very many options we support pinging an absurd amount of proxies or a bunch of websites.

Pingman uses the tokio runtime for its amazing tasks to run all the requests in parallel, so they have no delay between each of them.

# Why HTTP pinging isn't it slow?
This binary uses http pinging because it fits my use case, yes http pinging is pretty damn slow compared to raw TCP but raw TCP doesn't really test for anything at all for my use cases, I want to test how fast my client can send a message to the server THROUGH HTTP I don't really care if it's not the fastest because all pings really are is a test to see how fast the connection is for your use case.

Also, pingman intends to support http proxies (and maybe socks5?) which cannot be done obviously through raw TCP.

# Note
These are http pings, so they are a bit slower than a typical raw TCP socket ping because of all the extra overhead of http, it also uses reqwest which is a relatively high level library to do the pings themselves, this could potentially slow down request times not sure though.
