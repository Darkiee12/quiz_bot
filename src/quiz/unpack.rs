use crate::utils::models::Question;
use crate::Error;
use rand::seq::SliceRandom;
use serde_json;
use std::io::Read;

pub fn unpack_quiz() -> Result<Vec<Question>, Error> {
    // let quiz_path = "./quiz.json";
    // let mut quiz_file = std::fs::File::open(quiz_path);
    // let mut contents = String::new();
    // quiz_file.read_to_string(&mut contents)?;

    // let mut quiz: Vec<Question> = match serde_json::from_str(&contents){
    //     Ok(quiz) => quiz,
    //     Err(e) => {
    //         serde_json::from_str(json()).unwrap()
    //     }
    // };
    let mut quiz: Vec<Question> =  serde_json::from_str(json()).unwrap();
    let mut rng = rand::thread_rng();
    quiz.shuffle(&mut rng);
    Ok(quiz)
}

fn json() -> &'static str{
    r#"
    [
  {
    "question": "What is the primary function of the data plane in the network layer?",
    "kind": "choices",
    "choices": [
      {
        "content": "Forwarding packets between network devices",
        "value": true
      },
      {
        "content": "Establishing connections between hosts",
        "value": false
      },
      {
        "content": "Managing network resources",
        "value": false
      },
      {
        "content": "Implementing network security protocols",
        "value": false
      }
    ]
  },
  {
    "question": "What is the advantage of using SDN in network layer management?",
    "kind": "choices",
    "choices": [
      {
        "content": "Limited scalability and flexibility",
        "value": false
      },
      {
        "content": "Higher latency and slower data transmission",
        "value": false
      },
      {
        "content": "Increased physical network devices required.",
        "value": false
      },
      {
        "content": "Simplified network management and configuration",
        "value": true
      }
    ]
  },
  {
    "question": "Which of the following devices operates at the data plane in the network layer?",
    "kind": "choices",
    "choices": [
      {
        "content": "Router",
        "value": true
      },
      {
        "content": "Switch",
        "value": false
      },
      {
        "content": "Hub",
        "value": false
      },
      {
        "content": "Firewall",
        "value": false
      }
    ]
  },
  {
    "question": "What is the drawback of the link state routing algorithm?",
    "kind": "choices",
    "choices": [
      {
        "content": "It requires frequent updates due to its dynamic nature.",
        "value": false
      },
      {
        "content": "It is susceptible to routing loops and count-to-infinity issues.",
        "value": false
      },
      {
        "content": "It is less scalable for large networks.",
        "value": true
      },
      {
        "content": "It does not support route summarization.",
        "value": false
      }
    ]
  },
  {
    "question": "Which protocol is commonly used for control plane communication between network devices?",
    "kind": "choices",
    "choices": [
      {
        "content": "Internet Protocol (IP)",
        "value": false
      },
      {
        "content": "Border Gateway Protocol (BGP)",
        "value": true
      },
      {
        "content": "Simple Network Management Protocol (SNMP)",
        "value": false
      },
      {
        "content": "Transmission Control Protocol (TCP)",
        "value": false
      }
    ]
  },
  {
    "question": "Which of the following is a benefit of SDN in terms of network scalability?",
    "kind": "choices",
    "choices": [
      {
        "content": "It limits the number of devices that can be connected to the network.",
        "value": false
      },
      {
        "content": "It requires manual configuration of network devices.",
        "value": false
      },
      {
        "content": "It allows for easy addition and removal of network devices.",
        "value": true
      },
      {
        "content": "It increases network complexity",
        "value": false
      }
    ]
  },
  {
    "question": "Which of the following is NOT an example of a control plane protocol used for routing in the Internet?",
    "kind": "choices",
    "choices": [
      {
        "content": "OSPF (Open Shortest Path First)",
        "value": false
      },
      {
        "content": "Border Gateway Protocol (BGP)",
        "value": false
      },
      {
        "content": "Routing Informatin Protocol (RIP)",
        "value": false
      },
      {
        "content": "Internet Control Message Protocol",
        "value": true
      }
    ]
  },
  {
    "question": "What is the drawback of the distance vector routing algorithm?",
    "kind": "choices",
    "choices": [
      {
        "content": "It requires frequent updates due to its dynamic nature.",
        "value": false
      },
      {
        "content": "It is susceptible to routing loops and count-to-infinity issues.",
        "value": true
      },
      {
        "content": "It is less scalable for large networks.",
        "value": false
      },
      {
        "content": "It does not support route summarizatoon.",
        "value": false
      }
    ]
  },
  {
    "question": "Which protocol operates at the network layer to deliver packets across different networks?",
    "kind": "choices",
    "choices": [
      {
        "content": "IP (Internet Protocol)",
        "value": true
      },
      {
        "content": "TCP (Transmission Control Protocol)",
        "value": false
      },
      {
        "content": "ICMP (Internet Control Message Protocol)",
        "value": false
      },
      {
        "content": "ARP (Address Resolution Protocol)",
        "value": false
      }
    ]
  },
  {
    "question": "What is the purpose of interfaces in a router?",
    "kind": "choices",
    "choices": [
      {
        "content": "To provide physical connectivity to network devices",
        "value": true
      },
      {
        "content": "To perform packet forwarding and routing decisions",
        "value": false
      },
      {
        "content": "To store configuration files and routing tables",
        "value": false
      },
      {
        "content": "To manage the router's operating system and control plane processes",
        "value": false
      }
    ]
  },
  {
    "question": "What is the main concept behind SDN?",
    "kind": "choices",
    "choices": [
      {
        "content": "Centralized control and programmability of network infrastructure",
        "value": true
      },
      {
        "content": "Decentralized control and manual configuration of network devices",
        "value": false
      },
      {
        "content": "Hardware-based networking without sotiware abstraction",
        "value": false
      },
      {
        "content": "Decentralized control and programmability of network infrastructure",
        "value": false
      }
    ]
  },
  {
    "question": "What is tunneling in the context of IPv6 transition?",
    "kind": "choices",
    "choices": [
      {
        "content": "Translating IPv6 addresses to IPv4 addresses.",
        "value": false
      },
      {
        "content": "Encapsulating IPv6 packets within IPv4 packets for transport across IPv4 networks",
        "value": true
      },
      {
        "content": "Assigning temporary IPv6 addresses to devices during the transition period.",
        "value": false
      },
      {
        "content": "Allocating IPv6 addresses to IPv4-only devices.",
        "value": false
      }
    ]
  },
  {
    "question": "ICMP is closely associated with which protocol?",
    "kind": "choices",
    "choices": [
      {
        "content": "IP (Internet Protocol)",
        "value": true
      },
      {
        "content": "TCP (Transmission Control Protocol)",
        "value": false
      },
      {
        "content": "UDP (User Datagram Protocol)",
        "value": false
      },
      {
        "content": "HTTP (Hypertext Transfer Protocol)",
        "value": false
      }
    ]
  },
  {
    "question": "Which component of a router is responsible for processing and forwarding data packets?",
    "kind": "choices",
    "choices": [
      {
        "content": "Routing Processor",
        "value": false
      },
      {
        "content": "Routing Logic",
        "value": false
      },
      {
        "content": "Switching Fabric",
        "value": true
      },
      {
        "content": "Forwarding Processor",
        "value": false
      }
    ]
  },
  {
    "question": "What is the structure of an IPv4 address?",
    "kind": "choices",
    "choices": [
      {
        "content": "Network portion and host portion",
        "value": true
      },
      {
        "content": "Prefix and suffix",
        "value": false
      },
      {
        "content": "Subnet mask and subnet ID",
        "value": false
      },
      {
        "content": "Class and network ID.",
        "value": false
      }
    ]
  },
  {
    "question": "How does a router share its link state information in the link state routing algorithm?",
    "kind": "choices",
    "choices": [
      {
        "content": "By broadcasting the information to all routers in the network",
        "value": false
      },
      {
        "content": "By multicasting the information to specific routers in the network",
        "value": false
      },
      {
        "content": "By sending the information directly to neighboring routers",
        "value": false
      },
      {
        "content": "By flooding the information to all routers in the network.",
        "value": true
      }
    ]
  },
  {
    "question": "What information does a router maintain in the link state database in the link state routing algorithm?",
    "kind": "choices",
    "choices": [
      {
        "content": "Routing table entries",
        "value": false
      },
      {
        "content": "Hop count to destination networks.",
        "value": false
      },
      {
        "content": "Neighbor router information",
        "value": false
      },
      {
        "content": "Topology information of the entire network",
        "value": true
      }
    ]
  },
  {
    "question": "How does a router share its routing table information in the distance vector routing algorithm?",
    "kind": "choices",
    "choices": [
      {
        "content": "By broadcasting the information to all routers in the network",
        "value": false
      },
      {
        "content": "By multicasting the information to specific routers in the network",
        "value": false
      },
      {
        "content": "By sending the information directly to neighboring routers",
        "value": true
      },
      {
        "content": "By flooding the information to all routers in the network",
        "value": false
      }
    ]
  },
  {
    "question": "Which of the following is a characteristic of the data plane in the network layer?",
    "kind": "choices",
    "choices": [
      {
        "content": "It determines the optimal path for packet delivery.",
        "value": false
      },
      {
        "content": "It manages network resources and bandwidth allocation.",
        "value": false
      },
      {
        "content": "It enforces security policies and access control.",
        "value": true
      },
      {
        "content": "It focuses on the efficient forwarding of packets.",
        "value": false
      }
    ]
  },
  {
    "question": "What is the disadvantage of using NAT?",
    "kind": "choices",
    "choices": [
      {
        "content": "It may increase network latency.",
        "value": true
      },
      {
        "content": "It weakens network security.",
        "value": false
      },
      {
        "content": "It requires additional network infrastructure.",
        "value": false
      },
      {
        "content": "All of above.",
        "value": false
      }
    ]
  },
  {
    "question": "What is the advantage of using the link state routing algorithm over the distance vector routing algorithm?",
    "kind": "choices",
    "choices": [
      {
        "content": "It has lower memory and processing requirements.",
        "value": false
      },
      {
        "content": "It is easier to implement and configure.",
        "value": false
      },
      {
        "content": "It provides faster convergence in large networks.",
        "value": true
      },
      {
        "content": "It is more suitable for stable network topologies.",
        "value": false
      }
    ]
  },
  {
    "question": "What is the purpose of the subnet mask in an IPv4 address?",
    "kind": "choices",
    "choices": [
      {
        "content": "To identify the network portion of the address.",
        "value": true
      },
      {
        "content": "To identify the host portion of the address.",
        "value": false
      },
      {
        "content": "To determine the class of the address.",
        "value": false
      },
      {
        "content": "To provide security for the network.",
        "value": false
      }
    ]
  },
  {
    "question": "Which protocol is commonly used in SDN to communicate between the control plane and data plane?",
    "kind": "choices",
    "choices": [
      {
        "content": "Border Gateway Protocol (BGP)",
        "value": false
      },
      {
        "content": "OpenFlow.",
        "value": true
      },
      {
        "content": "Transmission Control Protocol (TCP)",
        "value": false
      },
      {
        "content": "Internet Control Message Protocol (ICMP)",
        "value": false
      }
    ]
  },
  {
    "question": "Which classful address range is reserved for multicast addresses?",
    "kind": "choices",
    "choices": [
      {
        "content": "Class A",
        "value": false
      },
      {
        "content": "Class B",
        "value": false
      },
      {
        "content": "Class C",
        "value": false
      },
      {
        "content": "Class D.",
        "value": true
      }
    ]
  },
  {
    "question": "What is the advantage of using the distance vector routing algorithm over the link state routing algorithm?",
    "kind": "choices",
    "choices": [
      {
        "content": "It converges more quickly in large networks.",
        "value": false
      },
      {
        "content": "It requires less memory and processing resources.",
        "value": true
      },
      {
        "content": "It provides more accurate network topology information.",
        "value": false
      },
      {
        "content": "It supports more advanced routing features.",
        "value": false
      }
    ]
  },
  {
    "question": "Which field in the IPv4 header is used for fragmentation and reassembly of packets?",
    "kind": "choices",
    "choices": [
      {
        "content": "Source IP address",
        "value": false
      },
      {
        "content": "Destination IP address",
        "value": false
      },
      {
        "content": "Datagram length",
        "value": false
      },
      {
        "content": "Identifier.",
        "value": true
      }
    ]
  },
  {
    "question": "How does a router calculate the shortest path to a destination in the link state routing algorithm?",
    "kind": "choices",
    "choices": [
      {
        "content": "By comparing the hop count to the destination network",
        "value": false
      },
      {
        "content": "By using the Bellman-Ford algorithm",
        "value": false
      },
      {
        "content": "By running Dijkstra's algorithm on the link state database.",
        "value": true
      },
      {
        "content": "By consulting the routing table entries",
        "value": false
      }
    ]
  },
  {
    "question": "Which IPv4 address range is reserved for private networks?",
    "kind": "choices",
    "choices": [
      {
        "content": "10.0.0.0 - 10.255.255.255",
        "value": false
      },
      {
        "content": "172.16.0.0 - 172.31.255.255",
        "value": false
      },
      {
        "content": "192.168.0.0 - 192.168.255.255",
        "value": false
      },
      {
        "content": "All of the above.",
        "value": true
      }
    ]
  },
  {
    "question": "Which of the following is a limitation of SDN in the network layer?",
    "kind": "choices",
    "choices": [
      {
        "content": "Reduced network performance and increased latency",
        "value": false
      },
      {
        "content": "Inability to handle large-scale networks.",
        "value": false
      },
      {
        "content": "Limited support for network virtualization",
        "value": false
      },
      {
        "content": "Dependency on a centralized controller can be a single point of failure.",
        "value": true
      }
    ]
  },
  {
    "question": "Which routing algorithm is based on the concept of link-state information?",
    "kind": "choices",
    "choices": [
      {
        "content": "Distance Vector Routing Algorithm",
        "value": false
      },
      {
        "content": "Link State Routing Algorithm",
        "value": true
      },
      {
        "content": "Border Gateway Protocol (BGP)",
        "value": false
      },
      {
        "content": "Open Shortest Path First (OSPF)",
        "value": true
      }
    ]
  },
  {
    "question": "Which statement about NAT is true?",
    "kind": "choices",
    "choices": [
      {
        "content": "NAT is not compatible with firewalls.",
        "value": false
      },
      {
        "content": "NAT can translate IP addresses between different network protocols.",
        "value": false
      },
      {
        "content": "NAT can only be used in small networks, not large-scale deployments.",
        "value": false
      },
      {
        "content": "NAT can only translate IPv4 addresses, not IPv6 addresses.",
        "value": true
      }
    ]
  },
  {
    "question": "Which of the following is an example of an SDN controller?",
    "kind": "choices",
    "choices": [
      {
        "content": "OpenVPN",
        "value": false
      },
      {
        "content": "Wireshark",
        "value": false
      },
      {
        "content": "OpenDaylight.",
        "value": true
      },
      {
        "content": "Cisco IOS",
        "value": false
      }
    ]
  },
  {
    "question": "What is the role of the OpenFlow protocol in SDN?",
    "kind": "choices",
    "choices": [
      {
        "content": "It provides encryption and security for network traffic.",
        "value": false
      },
      {
        "content": "It enables communication between the SDN controller and network devices.",
        "value": true
      },
      {
        "content": "It establishes connections between hosts in different networks.",
        "value": false
      },
      {
        "content": "It handles the physical connectivity between network devices",
        "value": false
      }
    ]
  },
  {
    "question": "How is an IPv4 address written?",
    "kind": "choices",
    "choices": [
      {
        "content": "In binary format",
        "value": false
      },
      {
        "content": "In hexadecimal format",
        "value": false
      },
      {
        "content": "In octal format",
        "value": false
      },
      {
        "content": "In decimal format with dot-decimal notation",
        "value": true
      }
    ]
  },
  {
    "question": "How does SDN improve network agility in the network layer?",
    "kind": "choices",
    "choices": [
      {
        "content": "It increases network complexity and management overhead.",
        "value": false
      },
      {
        "content": "It allows for rapid deployment and reconfiguration of network services.",
        "value": true
      },
      {
        "content": "It limits the flexibility of network infrastructure.",
        "value": false
      },
      {
        "content": "It requires manual configuration of network devices.",
        "value": false
      }
    ]
  },
  {
    "question": "What is the control plane in computer networking?",
    "kind": "choices",
    "choices": [
      {
        "content": "The part of the network responsible for forwarding data packets",
        "value": false
      },
      {
        "content": "The part of the network responsible for controlling and managing network devices",
        "value": true
      },
      {
        "content": "The part of the network responsible for encrypting network traffic",
        "value": false
      },
      {
        "content": "The part of the network responsible for routing network traffic",
        "value": false
      }
    ]
  },
  {
    "question": "Which statement about ICMP is correct?",
    "kind": "choices",
    "choices": [
      {
        "content": "ICMP messages are always sent in response to specific events.",
        "value": true
      },
      {
        "content": "ICMP messages are transported over TCP.",
        "value": false
      },
      {
        "content": "ICMP provides reliable data transfer.",
        "value": false
      },
      {
        "content": "ICMP is primarily used for routing and forwarding packets.",
        "value": false
      }
    ]
  },
  {
    "question": "Which control plane function involves exchanging routing information between network devices?",
    "kind": "choices",
    "choices": [
      {
        "content": "Network monitoring",
        "value": false
      },
      {
        "content": "Network configuration",
        "value": false
      },
      {
        "content": "Routing table updates",
        "value": true
      },
      {
        "content": "Quality of Service (QoS) management",
        "value": false
      }
    ]
  },
  {
    "question": "Which of the following is an example of an addressing scheme used in the network layer?",
    "kind": "choices",
    "choices": [
      {
        "content": "IPv4 (Internet Protocol version 4)",
        "value": true
      },
      {
        "content": "HTTP (Hypertext Transfer Protocol)",
        "value": false
      },
      {
        "content": "DNS (Domain Name System)",
        "value": false
      },
      {
        "content": "FTP (File Transfer Protocol)",
        "value": false
      }
    ]
  },
  {
    "question": "In a network layer, what is the purpose of Quality of Service (QoS) mechanisms?",
    "kind": "choices",
    "choices": [
      {
        "content": "To prioritize certain types of network traffic",
        "value": true
      },
      {
        "content": "To encrypt data packets for secure transmission",
        "value": false
      },
      {
        "content": "To establish connections between hosts",
        "value": false
      },
      {
        "content": "To prevent unauthorized access to the network",
        "value": false
      }
    ]
  },
  {
    "question": "What is the maximum number of unique IPv4 addresses?",
    "kind": "choices",
    "choices": [
      {
        "content": "1 million",
        "value": false
      },
      {
        "content": "4 billion",
        "value": true
      },
      {
        "content": "16 million",
        "value": false
      },
      {
        "content": "256 million",
        "value": false
      }
    ]
  },
  {
    "question": "How does the switching fabric impact the performance of a router?",
    "kind": "choices",
    "choices": [
      {
        "content": "It provides physical connectivity to network devices.",
        "value": false
      },
      {
        "content": "It manages and controls the router's operating system.",
        "value": false
      },
      {
        "content": "It determines the maximum throughput and latency of the router.",
        "value": true
      },
      {
        "content": "It handles packet forwarding and routing decisions.",
        "value": false
      }
    ]
  },
  {
    "question": "What is the main advantage of using IPv6 transition mechanisms?",
    "kind": "choices",
    "choices": [
      {
        "content": "They allow for a seamless transition from IPv4 to IPv6 with minimal effort.",
        "value": false
      },
      {
        "content": "They enable communication between IPv4 and IPv6 networks without the need for any changes to network infrastructure.",
        "value": true
      },
      {
        "content": "They provide enhanced security features compared to IPv4.",
        "value": false
      },
      {
        "content": "They eliminate the need for dual-stack implementation.",
        "value": false
      }
    ]
  },
  {
    "question": "Which ICMP message type is used to test the reachability of a host or network (i.e. ping)?",
    "kind": "choices",
    "choices": [
      {
        "content": "Echo Request",
        "value": true
      },
      {
        "content": "Destination Unreachable",
        "value": false
      },
      {
        "content": "Time Exceeded",
        "value": false
      },
      {
        "content": "Redirect",
        "value": false
      }
    ]
  },
  {
    "question": "Which statement about BGP is correct?",
    "kind": "choices",
    "choices": [
      {
        "content": "It is a protocol used within a single autonomous system (AS)",
        "value": false
      },
      {
        "content": "It is a protocol used for routing within a local area network (LAN)",
        "value": false
      },
      {
        "content": "It is a protocol used for routing within a wide area network (WAN)",
        "value": false
      },
      {
        "content": "It is a protocol used for routing between different autonomous systems (AS)",
        "value": true
      }
    ]
  },
  {
    "question": "Which of the following is a characteristic of the distance vector routing algorithm?",
    "kind": "choices",
    "choices": [
      {
        "content": "It converges quickly in large networks.",
        "value": false
      },
      {
        "content": "It does not require periodic updates.",
        "value": false
      },
      {
        "content": "It is less resource-intensive compared to link state routing.",
        "value": false
      },
      {
        "content": "It is more suitable for stable network topologies.",
        "value": true
      }
    ]
  },
  {
    "question": "What are the main components of a router?",
    "kind": "choices",
    "choices": [
      {
        "content": "Routing Processor, Routing Logic, Input, Output",
        "value": false
      },
      {
        "content": "Routing Processor, Switching Fabric, Input, Output",
        "value": true
      },
      {
        "content": "Forwarding Processor, Switching Fabric, Input, Output",
        "value": false
      },
      {
        "content": "Forwarding Processor, Routing Logic, Input, Output",
        "value": false
      }
    ]
  },
  {
    "question": "Which network device is responsible for hosting the control plane functions in a computer network?",
    "kind": "choices",
    "choices": [
      {
        "content": "Modem",
        "value": false
      },
      {
        "content": "Switch",
        "value": false
      },
      {
        "content": "Router",
        "value": true
      },
      {
        "content": "Firewall",
        "value": false
      }
    ]
  },
  {
    "question": "ICMP is a protocol used for",
    "kind": "choices",
    "choices": [
      {
        "content": "Transporting data packets",
        "value": false
      },
      {
        "content": "Establishing network connections",
        "value": false
      },
      {
        "content": "Error reporting and diagnostic messages",
        "value": true
      },
      {
        "content": "Network address translation (NAT)",
        "value": false
      }
    ]
  },
  {
    "question": "What is the purpose of Network Address Translation (NAT) in IPv4?",
    "kind": "choices",
    "choices": [
      {
        "content": "To encrypt and secure IPv4 traffic.",
        "value": false
      },
      {
        "content": "To route IPv4 packets between networks",
        "value": false
      },
      {
        "content": "To assign unique IP addresses to devices within a network",
        "value": false
      },
      {
        "content": "To translate private IP addresses to public IP addresses for internet connectivity",
        "value": true
      }
    ]
  },
  {
    "question": "What is the function of ICMP (Internet Control Message Protocol) in the network layer?",
    "kind": "choices",
    "choices": [
      {
        "content": "To report errors and provide diagnostic information",
        "value": true
      },
      {
        "content": "To establish connections between hosts",
        "value": false
      },
      {
        "content": "To translate IP addresses into physical addresses",
        "value": false
      },
      {
        "content": "To route packets between networks",
        "value": false
      }
    ]
  },
  {
    "question": "Which component of a network device is responsible for executing control plane functions?",
    "kind": "choices",
    "choices": [
      {
        "content": "Network interface card (NIC)",
        "value": false
      },
      {
        "content": "Central processing unit (CPU)",
        "value": true
      },
      {
        "content": "Random Access Memory (RAM)",
        "value": false
      },
      {
        "content": "Switching Fabric",
        "value": false
      }
    ]
  },
  {
    "question": "Which device is typically responsible for performing NAT?",
    "kind": "choices",
    "choices": [
      {
        "content": "Modem",
        "value": false
      },
      {
        "content": "Hub",
        "value": false
      },
      {
        "content": "Switch",
        "value": false
      },
      {
        "content": "Router",
        "value": true
      }
    ]
  },
  {
    "question": "What is the default subnet mask for a Class C IPv4 address?",
    "kind": "choices",
    "choices": [
      {
        "content": "255.0.0.0",
        "value": false
      },
      {
        "content": "255.255.0.0",
        "value": false
      },
      {
        "content": "255.255.255.0",
        "value": true
      },
      {
        "content": "255.255.255.255",
        "value": false
      }
    ]
  },
  {
    "question": "Which routing algorithm is based on the concept of exchanging routing information between neighboring routers?",
    "kind": "choices",
    "choices": [
      {
        "content": "Distance Vector Routing Algorithm",
        "value": true
      },
      {
        "content": "Link State Routing Algorithm",
        "value": false
      },
      {
        "content": "Border Gateway Protocol (BGP)",
        "value": false
      },
      {
        "content": "Open Shortest Path First (OSPF)",
        "value": false
      }
    ]
  },
  {
    "question": "What does a router do in the network layer?",
    "kind": "choices",
    "choices": [
      {
        "content": "Forwards packets based on destination IP addresses.",
        "value": true
      },
      {
        "content": "Translates IP addresses into physical addresses.",
        "value": false
      },
      {
        "content": "Establishes connections between hosts.",
        "value": false
      },
      {
        "content": "Filters and blocks malicious network traffic.",
        "value": false
      }
    ]
  },
  {
    "question": "What is the primary reason for the development of IPv6?",
    "kind": "choices",
    "choices": [
      {
        "content": "Exhaustion of IPv4 addresses",
        "value": true
      },
      {
        "content": "Faster routing protocols",
        "value": false
      },
      {
        "content": "Enhanced security features",
        "value": false
      },
      {
        "content": "Improved quality of service (QoS)",
        "value": false
      }
    ]
  },
  {
    "question": "What does SDN stand for?",
    "kind": "choices",
    "choices": [
      {
        "content": "Software-Defined Networking",
        "value": true
      },
      {
        "content": "Software-Designed Networking",
        "value": false
      },
      {
        "content": "Software-Definition Network",
        "value": false
      },
      {
        "content": "Software-Distributed Networking",
        "value": false
      }
    ]
  },
  {
    "question": "Which statement about the link state routing algorithm is correct?",
    "kind": "choices",
    "choices": [
      {
        "content": "It is a distance-vector-based routing algorithm.",
        "value": false
      },
      {
        "content": "It uses only the hop count as the metric for path selection.",
        "value": false
      },
      {
        "content": "It requires routers to periodically broadcast their routing tables.",
        "value": false
      },
      {
        "content": "It provides a more accurate representation of network topology than distance vector routing algorithms.",
        "value": true
      }
    ]
  },
  {
    "question": "Which of the following is NOT a switching technique used in a router?",
    "kind": "choices",
    "choices": [
      {
        "content": "Shared Memory",
        "value": false
      },
      {
        "content": "Bus",
        "value": false
      },
      {
        "content": "Interconnection Network",
        "value": false
      },
      {
        "content": "Token",
        "value": true
      }
    ]
  },
  {
    "question": "How is an IPv6 address written?",
    "kind": "choices",
    "choices": [
      {
        "content": "In binary format",
        "value": false
      },
      {
        "content": "In hexadecimal format",
        "value": true
      },
      {
        "content": "In octal format",
        "value": false
      },
      {
        "content": "In decimal format with dot-decimal notation",
        "value": false
      }
    ]
  },
  {
    "question": "Which layer of the OSI model corresponds to the network layer?",
    "kind": "choices",
    "choices": [
      {
        "content": "Layer 1",
        "value": false
      },
      {
        "content": "Layer 2",
        "value": false
      },
      {
        "content": "Layer 3",
        "value": true
      },
      {
        "content": "Layer 4",
        "value": false
      }
    ]
  },
  {
    "question": "How many bits are used to represent an IPv4 address?",
    "kind": "choices",
    "choices": [
      {
        "content": "32",
        "value": true
      },
      {
        "content": "64",
        "value": false
      },
      {
        "content": "128",
        "value": false
      },
      {
        "content": "256",
        "value": false
      }
    ]
  },
  {
    "question": "ICMP stands for",
    "kind": "choices",
    "choices": [
      {
        "content": "Internet Configuration and Management Protocol",
        "value": false
      },
      {
        "content": "Internet Communication Monitoring Protocol",
        "value": false
      },
      {
        "content": "Internet Connection Management Protocol",
        "value": false
      },
      {
        "content": "Internet Control Message Protocol",
        "value": true
      }
    ]
  },
  {
    "question": "What is the purpose of the Address Resolution Protocol (ARP) in IPv4?",
    "kind": "choices",
    "choices": [
      {
        "content": "To map IP addresses to MAC addresses",
        "value": true
      },
      {
        "content": "To route IPv4 packets between networks",
        "value": false
      },
      {
        "content": "To assign unique IP addresses to devices within a network",
        "value": false
      },
      {
        "content": "To translate private IP addresses to public IP addresses for internet connectivity",
        "value": false
      }
    ]
  },
  {
    "question": "Which statement about Static NAT is correct?",
    "kind": "choices",
    "choices": [
      {
        "content": "It allows for one-to-many IP address translation.",
        "value": true
      },
      {
        "content": "It dynamically assigns IP addresses to hosts.",
        "value": false
      },
      {
        "content": "It is commonly used for small networks.",
        "value": false
      },
      {
        "content": "It provides better network security than Dynamic NAT",
        "value": false
      }
    ]
  },
  {
    "question": "Which algorithm is used by routers to determine the shortest path in the distance vector routing algorithm?",
    "kind": "choices",
    "choices": [
      {
        "content": "Bellman-Ford algorithm",
        "value": true
      },
      {
        "content": "Dijkstra's algorithm",
        "value": false
      },
      {
        "content": "Floyd-Warshall algorithm",
        "value": false
      },
      {
        "content": "Prim's algorithm",
        "value": false
      }
    ]
  },
  {
    "question": "Which protocol is commonly used for control plane communication in a Border Gateway Protocol (BGP) environment?",
    "kind": "choices",
    "choices": [
      {
        "content": "TCP (Transmission Control Protocol)",
        "value": true
      },
      {
        "content": "UDP (User Datagram Protocol)",
        "value": false
      },
      {
        "content": "ICMP (Internet Control Message Protocol)",
        "value": false
      },
      {
        "content": "SNMP (Simple Network Management Protocol)",
        "value": false
      }
    ]
  },
  {
    "question": "How does the link state routing algorithm handle network changes or failures?",
    "kind": "choices",
    "choices": [
      {
        "content": "By sending periodic updates to all routers in the network",
        "value": false
      },
      {
        "content": "By recalculating the shortest path using Dijkstra's algorithm.",
        "value": true
      },
      {
        "content": "By adjusting the hop count to affected networks",
        "value": false
      },
      {
        "content": "By initiating a routing table update process.",
        "value": false
      }
    ]
  },
  {
    "question": "Which protocol is commonly used for the dynamic allocation of IPv4 addresses?",
    "kind": "choices",
    "choices": [
      {
        "content": "Internet Protocol Security (IPSec)",
        "value": false
      },
      {
        "content": "Internet Control Message Protocol (ICMP)",
        "value": false
      },
      {
        "content": "Dynamic Host Configuration Protocol (DHCP).",
        "value": true
      },
      {
        "content": "Border Gateway Protocol (BGP)",
        "value": false
      }
    ]
  }
]

    "#
}