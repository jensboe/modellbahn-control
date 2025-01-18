# Example layout
The arrows doesn't matter.
C_3 leads to a bumper.
New trains should be railed on C_3b.

```mermaid
flowchart TB

subgraph A
    direction LR

    As((D_1a)):::reserved

    A_1a:::reserved
    A_1b:::reserved_stop
    A_2a
    A_2b["A_2b
    Train I"]:::blocked
    A_3a
    A_3b["A_3b
    Train II"]:::blocked
    A_a:::planned
    A_b:::planned
    A_c:::reserved
    A_d:::reserved

    Ae((B_1a)):::planned

    As   ---  A_d
    A_d  ===  A_c
    A_d  -.-  A_3a
    A_c  ==== A_1a
    A_c  -..- A_2a

    A_1a ---- A_1b
    A_2a ---- A_2b
    A_3a ---- A_3b

    A_1b ==== A_a
    A_2b ===  A_b
    A_b  -.-  A_a
    A_3b -.-  A_b

    A_a --- Ae
end
D_1a:::reserved
B_1a:::planned

subgraph C
    direction RL

    Cs((B_1a)):::planned

    C_a:::planned
    C_1a:::planned
    C_1b:::planned
    C_2a
    C_2b
    C_3a[["C_3a"]]
    C_3b["A_3b
    Train III"]:::blocked_powered
    C_3c:::reserved_powered
    C_b
    C_c:::reserved

    Ce((D_1a)):::reserved

    Cs   ---   C_a
    C_a  ====  C_1a
    C_a  -.-   C_2a
    C_1a ----- C_1b
    C_2a ----- C_2b
    C_3a ---   C_3b
    C_3b ---   C_3c
    C_1b ===   C_b
    C_2b -.-   C_b
    C_b  -.-   C_c
    C_3c ===   C_c
    C_c  ---   Ce
end

A --- D_1a
A --- B_1a
D_1a --- C
B_1a --- C

classDef reserved fill:#00F
classDef planned fill:#007
classDef reserved_powered stroke:#0F0,stroke-width: 10px,fill:#00F
classDef reserved_stop stroke:#A00,stroke-width: 10px,fill:#00F
classDef blocked_powered stroke:#0F0,stroke-width: 10px,fill:#F00
classDef blocked fill:#A00
    
```

**Legend:**
- **Blue**: Reserved for a moving train
- **Dark Blue**: Planned route for a standing train
- **Green Stroke**: Powered track
- **Red Stroke**: Reserved destination (and stop location) of a moving train
- **Red**: Track blocked by a standing train
- **Green Stroke with Red Fill**: Track blocked by a moving train
- **Dark**: Free track
- **Normal Arrow**: Connection of a track
- **Bold Arrow**: Active connection of a switch
- **Dotted Arrow**: Inactive connection of a switch