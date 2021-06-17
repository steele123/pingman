# pingman
Pingman is a very fast pinger with an extremely verbose cli interface with very many options we support pinging an absurd amount of proxies or a bunch of websites.

Pingman uses the tokio runtime for its amazing tasks to run all the requests in parallel, so they have no delay between each of them.

# Note
These are http pings so they are a bit slower than a typical raw TCP socket ping because of all the extra overhead of http, it also uses reqwest which is a relatively high level library to do the pings themselves, this could potentially slow down request times not sure though.
