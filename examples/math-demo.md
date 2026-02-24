# Math Rendering Demo

This page demonstrates greymd's math rendering when built with `--features math`.

## Inline Math

The quadratic formula is $x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$, one of the most
well-known formulas in algebra.

Euler's identity $e^{i\pi} + 1 = 0$ connects five fundamental constants.

A matrix element $a_{ij}$ uses subscripts.

## Display Math

The Gaussian integral:

$$
\int_{-\infty}^{\infty} e^{-x^2} \, dx = \sqrt{\pi}
$$

Maxwell's equations in differential form:

$$
\nabla \cdot \mathbf{E} = \frac{\rho}{\varepsilon_0}
$$

$$
\nabla \times \mathbf{B} = \mu_0 \mathbf{J} + \mu_0 \varepsilon_0 \frac{\partial \mathbf{E}}{\partial t}
$$

A single-line display equation (opening `$$` must be at start of line):

$$E = mc^2$$

## Edge Cases

Currency values like $5 and $10 are **not** treated as math (space after `$`).

An escaped dollar sign \$100 renders literally.

Inline code takes priority: `$not math$` stays as code.

Fenced code blocks also take priority:

```
$$
This is not math, it's a code block.
$$
```

## Without Math Feature

When built without `--features math`, the raw LaTeX delimiters are preserved:
`$...$` and `$$...$$` pass through unchanged.
