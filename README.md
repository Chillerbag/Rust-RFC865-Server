# RFC 865 - QOTD

## Overview
This project is an implementation of RFC 865 - QOTD. It implements a QOTD server using TCP that can be easily hosted.
It accepts requests on port 17, as per specfication.
I have also included some niceties, such as IP Throttling, because I designed this to run on my home internet. Change if desired in .env!
Read about this RFC here: https://datatracker.ietf.org/doc/html/rfc865

## Development
This is a work in progress. Theres lots of bugs and TODOS but the basic functionality is complete.
A lot the admin commands "work" but dont do a great deal due to the choice to use .env, which was stupid and needs to be refactored. Probably will make it JSON.
Here are the remaining features to add:
rate limiting 
shutdown command
reboot command
JSON rewrite of adm command changes
writing to stream to send success or failure of admin commands

Also, I don't use HTTP headers. I dont care for making this a REST API and having POST body etc, it is as simple as possible. For that reason, with curlz you need to use flag --http0.9

## Bugs
For some reason, netcat makes you press enter to receive the quote? I dont know why. Curl doesn't. I reccomend using curl.

## Usage
**Getting a quote**:
</br>
`# With curl:`
</br>
`
curl --http0.9 127.0.0.1:9000
`
</br>
</br>
`# With netcat:`
</br>
`
nc 127.0.0.1 9000
`
</br>
</br>
**Sending adm commands**
</br>
`echo 'pw:"admin"|addquote:"This is a new quote" "Author Name"' | nc 127.0.0.1 9000
`

** adm command descriptions **
TODO


