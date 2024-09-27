# closer_line_prorotype
Test of closer line concept

The data on main, is the one from: https://trac.osgeo.org/postgis/timeline?from=2024-09-25T03%3A29%3A05-07%3A00&precision=second

The issue is that how to get the closer line from a point, from the given ones, we could use GEOS and Postgis for the calculations, with this result:

PostGIS:

 A-Q: 9.716835623285099e-05

 B-Q: 9.716835623092273e-05

GEOS:

 A-Q: 9.7168356230588148e-05

 B-Q: 9.7168356230827946e-05
 
So, for Postgis, B is the closer one, while for GEOS A is the closer one, the lines and points are so close that make it very hard to know which is the right answer, due to this there is a sample_perfect.py file, which runs all the computation steps in order to get the right result.
 
The closer line is the first segment of A, this does not imply Postgis is wrong, just there is a point with floats aproximation that a small change even in how the operation are done could change this result.

The real mindistance is 9.71683562296436e-05.

If we look good, all the results are relativily far from that result, the % of error is very low, the warning issue is how close the values between all the comparatives are very close to each other, but match for GEOS, for this case.

So, if want to improve this, is possible use some tricks which is tested here.

## Playgroud

To play this, we need to set some rules, I'll follow this ones.

- Avoid do much operations as we can
- Ideally never divide numbers, computers does not like it
- For closer distance, we does not need the full formula, just (x1 - x2)^2 + (y1 - y2)^2, is not the real distance, but for "closer" so keep a ranking of lines it works great, we can even avoid sqrt

To follow most of above, I decided for this playground use instead of floats a new type, just store the numerator and denominator, the point of do this is avoid divisions, and in case we need things like a > b or similar, we can always calculate using the nominators num1*div2 - num2*div1 >= 0, with this type of trick we can skip operations that we does not need for the ranking, and keep avoiding divide the numbers.

The Rust code does this, just a new type called Value, and play with numerators and denominators.

Do this needs to keep always numerators and denominators like this? the asnwer is no, the idea is, for a given problem, represent a concept like here, a distance with other numbers (or a set of them), which could helps to infer the relations like ">" or "<" skipping most operations as we can.

So, let see the distance from the real value to each solution:

Postgis: 1.279128170571242e-15

GEOS:    0.9445433801422154e-15

Playground: 0.22884797355737785e-15

The result for the playground is same as GEOS, A is the closer one! and even has a lower error taking as reference the other options.

Still we need to take some considerations.

- The proposed concept will reduce the actual case, but there will be always other cases with smaller floats where this will fails
- Avoid divide and multiply to keep numbers resolution can set a limit on how big the numbers can be, possible overflow there!

Thx!
