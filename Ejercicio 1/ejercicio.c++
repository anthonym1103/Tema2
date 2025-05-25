#include <iostream>
#include <vector>
#include <ctime>
#include <sstream>
#include <iomanip>
#include <cstdint>
#include <string>
#include <algorithm>
#include "sha.h"

// Resto de tu código original
struct Nodo
{
    std::string partida;
    std::vector<int> cuerpo;
    std::string firma;
    Nodo(std::string p, std::vector<int> c) : partida(p), cuerpo(c)
    {
        std::string concat = partida + " ";
        for (int num : cuerpo)
            concat += std::to_string(num) + " ";
        firma = sha256(concat);
    }
};

std::vector<Nodo> generar_lista(int n, int k)
{
    std::vector<Nodo> lista;
    time_t now = time(0);
    std::string partida_actual = sha256(std::ctime(&now));

    for (int i = 0; i < n; i++)
    {
        std::vector<int> cuerpo;
        for (int j = 0; j < k; j++)
            cuerpo.push_back(rand() % 100000 + 1);
        Nodo nodo(partida_actual, cuerpo);
        lista.push_back(nodo);
        partida_actual = nodo.firma;
    }
    return lista;
}

int main()
{
    srand(time(0));
    auto lista = generar_lista(3, 4);
    for (auto &nodo : lista)
    {
        std::cout << "Partida: " << nodo.partida.substr(0, 64) << "...\n";
        std::cout << "Cuerpo: ";
        for (int num : nodo.cuerpo)
            std::cout << num << " ";
        std::cout << "\nFirma: " << nodo.firma.substr(0, 64) << "...\n\n";
    }
    return 0;
}
