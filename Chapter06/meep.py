from random import random

class Meep:
    def exclaim(self):
        print("Holla!")

def probablyMakeMeep():
    if random() > 0.1:
        return Meep()

while True:
    meep = probablyMakeMeep()
    meep.exclaim()



