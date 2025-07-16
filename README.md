# Lab2-Conways-Game-Of-Life

![Conway's Game of Life Demo](./Simulacion.gif)

## Descripción

Proyecto del laboratorio 2 de Gráficas por Computadora. Implementa el autómata celular de Conway usando:
- Rust
- Raylib para ventanas
- Rand para patrones aleatorios

## Requisitos cumplidos

- Uso exclusivo de función `point()` para renderizado
- Función `get_color()` implementada
- Framebuffer 150x150 escalado a 800x600
- Algoritmo de Conway completo
- Topología toroidal
- Más de 10 patrones diferentes

## Reglas de Conway

1. Célula viva con menos de 2 vecinos muere (subpoblación)
2. Célula viva con 2-3 vecinos sobrevive
3. Célula viva con más de 3 vecinos muere (sobrepoblación)
4. Célula muerta con exactamente 3 vecinos vive (reproducción)

## Patrones implementados

**Patrones estáticos:**
- Block (cuadrado 2x2)
- Beehive (hexágono)

**Osciladores:**
- Blinker (período 2)
- Pulsar (período 3) 
- Pentadecathlon (período 15)

**Naves espaciales:**
- Glider (se mueve diagonalmente)
- Lightweight, Middleweight, Heavyweight Spaceships

**Otros:**
- Gosper Glider Gun (genera gliders infinitamente)
- Random Soup (áreas aleatorias)

## Como usar

```bash
cd Lab2-Conways-Game-Of-Life/game-of-life
cargo run
```

## Configuración

```rust
const GRID_WIDTH: usize = 150;
const GRID_HEIGHT: usize = 150;
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
```

La velocidad se puede cambiar en la línea donde dice `frame_count % 8` (números más pequeños = más rápido).


## Autor

Milton Polanco - 23471  
Universidad del Valle de Guatemala

---