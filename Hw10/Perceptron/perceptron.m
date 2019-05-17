% perceptron(p, f, W, b)
%
% a = perceptron returns the output of a perceptron given a
% function, a set of weights, a set of biases and an input.
function a = perceptron(p, f, W, b)
	n = W * p + b;
	a = arrayfun(f, n);
end