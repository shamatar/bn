#![feature(test)]
extern crate bincode;
extern crate bn;
extern crate rand;
extern crate test;

use bn::*;
use bincode::SizeLimit::Infinite;
use bincode::rustc_serialize::{decode, encode};

const SAMPLES: usize = 30;

macro_rules! benchmark(
    ($name:ident, $input:ident($rng:ident) = $pre:expr; $post:expr) => (
        #[bench]
        fn $name(b: &mut test::Bencher) {
            let $rng = &mut rand::thread_rng();
            let $input: Vec<_> = (0..SAMPLES).map(|_| $pre).collect();

            let mut c = 0;

            b.iter(|| {
                c += 1;

                let $input = &$input[c % SAMPLES];

                $post
            });
        }
    )
);

benchmark!(g1_serialization,
           input(rng) = G1::random(rng);

           encode(input, Infinite).unwrap()
);

benchmark!(g1_serialization_normalized,
           input(rng) = {let mut tmp = G1::random(rng); tmp.normalize(); tmp};

           encode(input, Infinite).unwrap()
);

benchmark!(g2_serialization,
           input(rng) = G2::random(rng);

           encode(input, Infinite).unwrap()
);

benchmark!(g2_serialization_normalized,
           input(rng) = {let mut tmp = G2::random(rng); tmp.normalize(); tmp};

           encode(input, Infinite).unwrap()
);

benchmark!(g1_deserialization,
           input(rng) = {encode(&G1::random(rng), Infinite).unwrap()};

           decode::<G1>(input).unwrap()
);

benchmark!(g2_deserialization,
           input(rng) = {encode(&G2::random(rng), Infinite).unwrap()};

           decode::<G2>(input).unwrap()
);

benchmark!(fr_addition,
           input(rng) = (Fr::random(rng), Fr::random(rng));

           input.0 + input.1
);

benchmark!(fr_subtraction,
           input(rng) = (Fr::random(rng), Fr::random(rng));

           input.0 - input.1
);

benchmark!(fr_multiplication,
           input(rng) = (Fr::random(rng), Fr::random(rng));

           input.0 * input.1
);

benchmark!(fr_inverses,
           input(rng) = Fr::random(rng);

           input.inverse()
);

benchmark!(g1_addition,
           input(rng) = (G1::random(rng), G1::random(rng));

           input.0 + input.1
);

benchmark!(g1_subtraction,
           input(rng) = (G1::random(rng), G1::random(rng));

           input.0 - input.1
);

benchmark!(g1_scalar_multiplication,
           input(rng) = (G1::random(rng), Fr::random(rng));

           input.0 * input.1
);

benchmark!(g2_addition,
           input(rng) = (G2::random(rng), G2::random(rng));

           input.0 + input.1
);

benchmark!(g2_subtraction,
           input(rng) = (G2::random(rng), G2::random(rng));

           input.0 - input.1
);

benchmark!(g2_scalar_multiplication,
           input(rng) = (G2::random(rng), Fr::random(rng));

           input.0 * input.1
);

benchmark!(fq12_scalar_multiplication,
           input(rng) = {
               let g1_1 = G1::random(rng);
               let g2_1 = G2::random(rng);

               let g1_2 = G1::random(rng);
               let g2_2 = G2::random(rng);

               (pairing(g1_1, g2_1), pairing(g1_2, g2_2))
           };

           input.0 * input.1
);

benchmark!(fq12_exponentiation,
           input(rng) = ({
               let g1 = G1::random(rng);
               let g2 = G2::random(rng);

               pairing(g1, g2)
           }, Fr::random(rng));

           input.0.pow(input.1)
);

benchmark!(perform_pairing,
           input(rng) = (G1::random(rng), G2::random(rng));

           pairing(input.0, input.1)
);

#[bench]
fn pair_one(b: &mut test::Bencher) {
    let num_points = 1;
    let rng = &mut rand::thread_rng();
    let input: Vec<_> = (0..num_points).map(|_| (G1::random(rng), G2::random(rng))).collect();
    let mut g1_points = vec![];
    let mut g2_points = vec![];
    for pair in input.into_iter() {
        g1_points.push(pair.0);
        g2_points.push(pair.1);
    }

    let mut c = 0;

    b.iter(|| {
        multi_pairing(g1_points.clone(), g2_points.clone());
    });
}

#[bench]
fn pair_ten(b: &mut test::Bencher) {
    let num_points = 10;
    let rng = &mut rand::thread_rng();
    let input: Vec<_> = (0..num_points).map(|_| (G1::random(rng), G2::random(rng))).collect();
    let mut g1_points = vec![];
    let mut g2_points = vec![];
    for pair in input.into_iter() {
        g1_points.push(pair.0);
        g2_points.push(pair.1);
    }

    let mut c = 0;

    b.iter(|| {
        multi_pairing(g1_points.clone(), g2_points.clone());
    });
}