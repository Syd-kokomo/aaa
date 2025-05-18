//Crear un programa que calcule el conjunto solución de una ecuacion cuadratica

use std::io::stdin;
use num::Complex;
struct Coeficientes {
    a: f64,
    b: f64,
    c: f64
}

fn main() { 
    let mut input = String::new();

    println!("Dada una ecuacion cuadratica de la forma ax^2+bx+c=0");
    println!("Escriba los valores de los coeficientes a, b, c");
    stdin().read_line(&mut input)
            .expect("No se pudo leer el dato");

    let a=input.trim()
            .parse()
            .expect("No se pudo convertir el numero");

    input.clear();
    stdin().read_line(&mut input)
            .expect("No se puedo leer el dato");
    let b=input.trim()
            .parse()
            .expect("Fallo en la conversion");
    
    input.clear();
    stdin().read_line(&mut input)
            .expect("No se pudo leer el dato");
    let c=input.trim()
            .parse()
            .expect("Fallo en la conversion");

    //Instancia de la struct
    let coef= Coeficientes{
        a, b, c
    };

    let discriminante = calcular_discriminante(coef.a, coef.b, coef.c);

    mostrar_resultados(coef.a, coef.b, discriminante);

}

fn calcular_discriminante(a: f64, b:f64, c:f64) -> f64{
    let b_cuadrado = b.powi(2);
    let discriminante = b_cuadrado-4.0*a*c;
    discriminante
}

fn soluciones_imaginarias(a:f64, b:f64, discriminante:f64){
    let raiz_complejos = 
    if discriminante < 0.0{
        Complex::new(0.0, (-discriminante).sqrt())
    } 
    else {
        Complex::new(discriminante.sqrt(), 0.0)
    };

    let num_complejo = Complex::new(-b, 0.0); 

    let x_1=(num_complejo + raiz_complejos) / Complex::new(a*2.0, 0.0);
    let x_2=(num_complejo - raiz_complejos) / Complex::new(a*2.0, 0.0);

    println!("Soluciones imaginarias: ");
    println!("x_1 = {}", x_1);
    println!("x_2 = {}", x_2);
}

fn solucion_unica(a:f64, b:f64, discriminante:f64){
    let x_1=(-b + (discriminante.sqrt())) / (2.0*a);

    println!("Solo existe una solucion real");
    println!("x_1 = {}", x_1);
}

fn soluciones_reales(a:f64, b:f64, discriminante:f64){
    let x_1=(-b + (discriminante.sqrt())) / (2.0*a);
    let x_2=(-b - (discriminante.sqrt())) / (2.0*a);
    println!("Soluciones reales: ");
    println!("x_1 = {}", x_1);
    println!("x_2 = {}", x_2);

}

fn mostrar_resultados(a:f64, b:f64, discriminante:f64){
    if discriminante < 0.0 {
        soluciones_imaginarias(a, b, discriminante);
    } 
    else if discriminante==0.0 {
        solucion_unica(a, b, discriminante);
    } 
    else {
        soluciones_reales(a, b, discriminante);
    }
}