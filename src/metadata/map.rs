/*
0 -
1 -
2 -
3 -
4 -
5 -
6 -
7 -
8 -
9 -
10 E3 Demo game begin
11 -
12 -
13 -
20 <game crash>
21 <game crash>
22 <game crash>
25 <game crash>
30 -
31 Hangar [2<- peyj workshop, 3<- beluga landing ]
32 Peyj's workshop
33 Lightouse
34 First boss map
35 Destroyed lighthouse
36 Invalid
37 Invalid
38 Invalid
39 Invalid
40 Invalid
41 Invalid
42 Invalid
46 Factory Entrance
50 Elevator room [1<- fence, 2<- electrical closet, 3<- tight spot, 4<- laboratory, 5<- elevator, 6 invalid, ]
51 Electrical closet [2<- elevator room]
52 Laboratory [2<- west wing]
53 West wing [2<- laboratory]
54 Upper hall [2<- grate, 3<-other grate, 4<-lasers, 5<-elevator]
55 X-ray verification
56 Closet
57 Nutripills vat [10<-in front of conveyor]
58 Loading dock [1<-normal entrance, 2<-closed bar corridor]
59 Computer room
60 Unused map(?) (weird, worth exploring)
61 Routing
62 Control room
63 Shipping room
64 -
70 -
72 Mammago garage
 42 -> Mdisk player
80 -
90 Hyllis
 1 -> Near factory
 2 -> Upside-down farmlands
 3 -> softlock run
 4 -> softlock run
91 Space (crash)
92 Selene (moon)
 1 -> Near top
 2 -> Moon surface
 42 -> mdisk player
93 <crash>
94 <crash>
100 -
110 Races 1 and 2
111 <crash>
114 Surveillance room
120 Slaughterhouse road
121 Slaughterhause quarter
122 Exterior moats
123 ""
130 Black isle
150 Main canal
151 Pedestrian district
152 Akuda bar
  1: main entrance
  2: hotel room
  42: bar mdisk
153 Ming Tzu's shop
   2: from pedestrian district
158 Iris Den
   2: from bar
   42: iris mdisk
154 Revolution scene
155 <crash (?)>
156 <crash>
160 <crash>
200 -
254 -
255 -
*/

pub const NAMES: [&str; 256] = [
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Lighthouse (E3 Demo)",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Peyj's workshop",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
    "Unknown",
];
