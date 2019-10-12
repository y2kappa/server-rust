


## Google Cloud Compute setup
    - Linux VM
    - Static IP + firewall rules setup
    - Rust + Git installed

## Network test

(Remote ssh into the Linux VM)
```
sanchez@instance-1:~$ nc -l -p 6123 -v
HIHIHI
Hi!
```

(Personal Mac)
```
[01:43:30 pm][sanchez]@sanchezs-MacBook-Pro ~
: nc -v 130.211.119.0 6123
HIHIHI
Hi!
```
