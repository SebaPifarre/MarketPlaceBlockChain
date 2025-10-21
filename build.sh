#!/bin/bash

compilar_contrato() {
    cargo +nightly-2024-05-20 contract build
}

mostrar_ayuda() {
    echo "Uso: $0 [opción]"
    echo "Opciones:"
    echo "  marketplace   Compila el contrato del marketplace."
    echo "  reportes      Compila el contrato para reportes."
    echo "  ambos         Compila ambos contratos (por defecto)."
    echo "  -h, --help    Muestra esta ayuda."
}

if [ "$1" == "-h" ] || [ "$1" == "--help" ]; then
    mostrar_ayuda
    exit 0
elif [ -z "$1" ] || [ "$1" == "ambos" ]; then
    echo "No se recibió parámetro. Compilando ambos contratos..."
    compilar_contrato
    status=$?
    if [ $status -ne 0 ]; then
        echo "Error: la compilación del marketplace falló (exit code $status). Abortando compilación de reportes."
        exit $status
    fi
    cd ReportesView && compilar_contrato
elif [ "$1" == "marketplace" ]; then
    compilar_contrato
elif [ "$1" == "reportes" ]; then
    cd ReportesView && compilar_contrato
else
    echo "error: Parámetro no válido."
    echo
    mostrar_ayuda
fi