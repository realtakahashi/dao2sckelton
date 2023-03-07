# What is this?
- This sckelton programs are basic implementaion of dao4commons.2.0.
- This program is highly versatile and provides an implementation for bi-directional data acquisition between contracts.

# Structure description

                                      -------------------
                                      |                 |
                                      |   DAO Core      |
                                      |                 |
                                      -------------------
                                              |
                                              V
                                      - - - - - - - - - -
                                      |                  |
                  ------------------  | default_contract | ------------------
                  |                   |                  |                  |   
                  |                   - - - - - - - - - -                   | 
                  v                                                         V      
         ----------------             - - - - - - - - - -           ----------------
        |                |            |                 |           |              |
        |   Function A   | <--------> | communication   | <-------> |  Function B  |
        |                |            |        base     |           |              |
        ------------------            - - - - - - - - - -           ----------------

- All functions use dao_core as the entry point. 
- And the contract where each function of dao is implemented has a common interface implemented by traits.
- Each function implemented as each contract makes it possible to implement various check functions and security by being called from the entry point dao.
- The communication_base contract is what makes it possible to exchange data between functions.
- By using the communication_base contract, each function is freed from all circular reference constraints, allowing data to be exchanged between functions, thereby realizing flexible contract design and optimal function division.





