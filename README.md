# Benchmarks and comparisons of krABMaga and leading ABM frameworks

### *Tested when our framework was named **Rust-AB** and with examples, so we provide that working version in this repository.*

**krABMaga** (previously named Rust-AB) is a discrete events simulation engine for developing ABM simulation that is written in Rust language.

krABMaga is designed to be a ready-to-use tool for the ABM community and for this reason the architectural concepts of the well-adopted MASON library were re-engineered to exploit the Rust peculiarities and programming model.

In this repository, we tested the majors ABM frameworks available:
- [**Agent.jl**](https://juliadynamics.github.io/Agents.jl/stable/)
- [**MASON**](https://cs.gmu.edu/~eclab/projects/mason/)
- [**MESA**](https://mesa.readthedocs.io/en/latest/)
- [**NetLogo**](https://ccl.northwestern.edu/netlogo/index.html)
- [**Repast**](https://repast.github.io)

In each directory, you can find tests and scripts to run the benchmarks.
Pay attention, you have to correctly install frameworks in order to run the benchmarks. Feel free to contact us if you have any questions about scripts and/or tests or if you can't reproduce our results.