**Modular exponentiation** is at the heart of the Diffie-Hellman key exchange.  
It is a **one-way function** that is **computationally infeasible** to reverse.

A prime p is usually chosen as the modulus and a **primitive root** as the base  
due to the special property that primitive roots are able to produce all the  
possible outcomes of mod p (i.e. the **resiude class** is the range [1, p - 1]).  

This program produces the residue class of a given base and modulus, and  
checks if the base is a primitive root of the modulus.

Other lessons:
- **exponentiation by squaring**  
Modular exponentiation with large numbers can get out of hand quickly if  
the base is raised to some large x. Exponentiation by squaring is a recursive  
 solution which I use here.




