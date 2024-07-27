# Systemd integration

A Bongonet server doesn't depend on systemd but it can easily be made into a systemd service.

```ini
[Service]
Type=forking
PIDFile=/run/bongonet.pid
ExecStart=/bin/bongonet -d -c /etc/bongonet.conf
ExecReload=kill -QUIT $MAINPID
ExecReload=/bin/bongonet -u -d -c /etc/bongonet.conf
```

The example systemd setup integrates Bongonet's graceful upgrade into systemd. To upgrade the bongonet service, simply install a version of the binary and then call `systemctl reload bongonet.service`.
