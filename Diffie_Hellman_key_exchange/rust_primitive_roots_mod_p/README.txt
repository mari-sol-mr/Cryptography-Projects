Modular exponentitaion is at the heart of the Diffie-Hellman key exchange.
It is a one-way function that is computationally infeasible to reverse.

A prime p is usually chosen as the modulus and a primitive root as the base
due to the special property that primitive primes are able to produce all the
possible outcomes of mod p (i.e. the resiude class is the range [1, p - 1]).

This program produces the residue class of a given base and modulus, and checks if 
the base is a primitive root of the modulus.

Other lessons:

Modular exponentitaion with large numbers can get out of hand quickly if the base raised to
some large x becomes too large. One solution for this is the recursive method "exponentiation
by squaring" which I use here.




