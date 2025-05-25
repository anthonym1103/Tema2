import hashlib
import random
from datetime import datetime

class Nodo:
    def __init__(self, partida, cuerpo):
        self.partida = partida
        self.cuerpo = cuerpo
        self.firma = hashlib.sha256((partida + " " + " ".join(map(str, cuerpo))).encode()).hexdigest()

def generar_lista(n, k):
    lista = []
    fecha = datetime.now().strftime("%d/%m/%Y %H:%M")
    partida_actual = hashlib.sha256(fecha.encode()).hexdigest()
    
    for _ in range(n):
        cuerpo = [random.randint(1, 100000) for _ in range(k)]
        nodo = Nodo(partida_actual, cuerpo)
        lista.append(nodo)
        partida_actual = nodo.firma
    
    return lista

# Ejemplo para n=3, k=4
lista = generar_lista(3, 4)
for nodo in lista:
    print(f"Partida: {nodo.partida[:64]}\nCuerpo: {nodo.cuerpo}\nFirma: {nodo.firma[:64]}\n")
