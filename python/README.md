# Python interface for TMSN

This module creates the network connection in a cluster for the TMSN-like communication.

Please see the demo in [the example Jupyter notebook](../examples/python-demo.ipynb).

## Usages

### Use a network

There are two methods:

- `network.send(packet)`: send out a packet
- `network.recv()`: try to receive a packet. If no new packet is received, it returns an empty list (i.e., [])

### Create a network

There are three methods:

- `start_network`: start a two-way connected network from the current machine
- `start_network_only_send`: create a one-way, send-only network from the current machine
- `start_network_only_recv`: create a one-way, receive-only network from the current machine


The method `start_network` starts a broadcast network using a subscription list.

Example:

```
network = start_network("machine_name", ["remote_ip_1", "remote_ip_2"], 8080)
network.send(data)
received_data = network.receive()
```

The network recieves as input a sender and a receiver of two channels, respectively,
one for incoming packets and the other for outgoing packets.

Each machine maintains a list of subscriptions. The list defines
the IPs that this machine is listening to.
Initially, this list is provided as the parameter `init_remote_ips`
of the function `start_network`.

The network structure between the machines are decided by your program, specifically by
explicitly setting the list of IPs to be subscribed from each machine.

#### Parameters

`start_network(name, init_remote_ips, port)`

* `name` - the local computer name.
* `init_remote_ips` - a list of IPs to which this computer makes a connection initially.
* `port` - the port number that the machines in the network are listening to.
`port` has to be the same value for all machines.


## Design

The local computer only connects to the computers specificed by the
`init_remote_ips` list in the function parameters (neighbors), and *receive* data from
these computers.

Specifically, a **Receiver** is created for each neighbor. The connection is initiated by the
Receiver. The number of Receivers on a computer is always equal to the number of neighbors.
On the other end, only one **Sender** is created for a computer, which send data to all other
computers that connected to it.



