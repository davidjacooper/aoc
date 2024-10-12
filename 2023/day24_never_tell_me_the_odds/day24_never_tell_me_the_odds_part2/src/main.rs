use std::time::Instant;
use decimal::d128;

fn main()
{
    let file = std::env::args().nth(1).unwrap_or("input.txt".to_string());
    match std::fs::read_to_string(&file)
    {
        Err(err) => println!("Cannot read '{file}': {err}"),
        Ok(content) =>
        {
            let start = Instant::now();
            if let Err(err) = never_tell_me_the_odds_part2(&content)
            {
                println!("ERROR: {}", err);
            }
            println!("---\ntime: {:?}", Instant::now().duration_since(start));
        }
    }
}

/*
 * This broke my brain, and was the only problem I couldn't get working on the day of release.
 * Eventually I was able to get it working, thanks to some insight from Reddit.
 *
 * I'd pieced together some of the important insight, but the use of planes in the working out
 * hadn't occurred to me. See comments below for details.
 *
 * Note that we only need 4 lines of input to arrive at the answer, and this is an O(1) procedure.
 *
 * Also note that the built-in f64 appears to have insufficient precision for this purpose; hence
 * 'decimal::d128'.
 */


#[derive(Clone,Debug)]
struct Coord
{
    x: d128, y: d128, z: d128
}

#[derive(Clone,Debug)]
struct Hailstone
{
    pos: Coord, vel: Coord
}


fn never_tell_me_the_odds_part2(content: &str) -> Result<(),&str>
{
    let mut hailstones: Vec<Hailstone> =
        content
        .trim_end()
        .lines()
        .take(4) // No need for the rest of the data
        .map(|line|
        {
            let coords: Vec<d128> =
                line
                .split(" @ ")
                .flat_map(|s| s.split(", "))
                .map(|s| s.trim().parse().unwrap())
                .collect();

            Hailstone {
                pos: Coord {x: coords[0], y: coords[1], z: coords[2]},
                vel: Coord {x: coords[3], y: coords[4], z: coords[5]}
            }
        })
        .collect();


    /*
     * 1. Transform all coordinates to a reference frame relative to hailstone #0 (H0). That is, we
     * centre the entire coordinate system around H0, so that H0 itself is fixed, unmoving, at the
     * origin.
     */

    let h0 = hailstones[0].clone();
    println!("Hailstones (RF shifted):");
    for h in hailstones.iter_mut()
    {
        h.pos.x -= h0.pos.x;
        h.pos.y -= h0.pos.y;
        h.pos.z -= h0.pos.z;
        h.vel.x -= h0.vel.x;
        h.vel.y -= h0.vel.y;
        h.vel.z -= h0.vel.z;
        println!("  {:?}", h);
    }


    /*
     * 2. Find the equation of a plane intersecting H0 and H1. The rock must pass through the
     * origin (H0), and it must also pass _somewhere_ through the line of H1's movement, so the
     * plane represents all rock trajectories possible given H0 and H1.
     *
     * To find the plane, we solve for 'a' and 'b' in the equation z = ax + by. (We know the
     * equation cannot have an additional constant term, because the plane passes through the
     * origin.)
     *
     * To get 'a' and 'b', we substitute in values of x and y known to be on the plane. This
     * technically includes H0, but we can't solve anything with zero. So, we pick points from the
     * line of H1, at t=0 and t=1, for simplicity.
     *
     * At t=0:
     *
     *   h1.pos.z = a * h1.pos.x + b * h1.pos.y
     *   => equation for a = ... (in terms of b and various constants)
     *
     * At t=1:
     *
     *   h1.pos.z + h1.vel.z = a * (h1.pos.x + h1.vel.x) + b * (h1.pos.y + h1.vel.y)
     *   => equation for b = .... (in terms of a and various constants)
     *   => substituting in 'a', we can find 'b' in terms of only known values.
     *
     */
    let h1 = &hailstones[1];
    let x_rel = (h1.pos.x + h1.vel.x) / h1.pos.x;
    let b = (h1.pos.z + h1.vel.z - h1.pos.z * x_rel) / (h1.pos.y + h1.vel.y - h1.pos.y * x_rel);
    let a = (h1.pos.z - b * h1.pos.y) / h1.pos.x;

    println!("a={a},\nb={b}");


    /*
     * 3. Find the times at which H2 and H3 intersect the plane. We are promised that there must be
     * an intersection, as the plane represents everywhere that the rock can travel (given H0 and H1).
     *
     * Hailstone coordinates are given parametrically, in terms of tₙ (for hailstone n). By
     * substituting H2's x, y and z coordinates (as functions of t₂), into the plane equation, we
     * derive an equation for t₂, and similarly for t₃:
     *
     *   h2.pos.z + t3 * h2.vel.z = a * (h2.pos.x + t3 * h2.vel.x) + b * (h2.pos.y + t3 * h2.vel.y)
     *   => equation for t2 = ... (in terms of a, b and other known values)
     *
     */

    let h2 = &hailstones[2];
    let h3 = &hailstones[3];
    let t2 = (h2.pos.z - a * h2.pos.x - b * h2.pos.y) / (a * h2.vel.x + b * h2.vel.y - h2.vel.z);
    let t3 = (h3.pos.z - a * h3.pos.x - b * h3.pos.y) / (a * h3.vel.x + b * h3.vel.y - h3.vel.z);


    /*
     * 4. For H2 and H3, find the (x,y,z) coordinates where each intersects the plane. We just plug
     * the calculated values of t₂ and t₃ straight into the basic parametric x/y/z equations.
     */
    let (x2, y2, z2) = (h2.pos.x + t2 * h2.vel.x, h2.pos.y + t2 * h2.vel.y, h2.pos.z + t2 * h2.vel.z);
    let (x3, y3, z3) = (h3.pos.x + t3 * h3.vel.x, h3.pos.y + t3 * h3.vel.y, h3.pos.z + t3 * h3.vel.z);

    println!("t2={t2} @ ({x2}, {y2}, {z2})");
    println!("t3={t3} @ ({x3}, {y3}, {z3})");


    /*
     * 5. Deduce the rock's (x,y,z) velocity. We just scale the distance between the H2/H3
     * intersection points by the time between the intersections.
     */
    let (rvx, rvy, rvz) = (
        (x3 - x2) / (t3 - t2),
        (y3 - y2) / (t3 - t2),
        (z3 - z2) / (t3 - t2));


    /*
     * 6. Deduce the rock's initial position. At time 0, the rock must have been t₂ times the rock's
     * velocity "less than" the H2 intersection point (so that it would take t₂ time to get from
     * the initial position to that point.)
     */
    let mut r = Hailstone {
        vel: Coord { x: rvx, y: rvy, z: rvz },
        pos: Coord {
            x: x2 - rvx * t2,
            y: y2 - rvy * t2,
            z: z2 - rvz * t2,
        }
    };
    println!("\nRock (RF shifted): {:?}", r);


    /*
     * 7. Restore the coordinate system. We've been working in a coordinate system relative to H0,
     * so we must add the coordinates of H0 to derive the rock's "real" coordinates.
     */
    r.pos.x += h0.pos.x;
    r.pos.y += h0.pos.y;
    r.pos.z += h0.pos.z;
    r.vel.x += h0.vel.x;
    r.vel.y += h0.vel.y;
    r.vel.z += h0.vel.z;
    println!("\nRock (actual): {:?}", r);

    println!("Sum of rock position coords = {}", r.pos.x + r.pos.y + r.pos.z);
    Ok(())
}

