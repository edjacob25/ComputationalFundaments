% train(P, T, f, maxIterations, seed)
%
% [W, b] = train trains a perceptron and returns the weights and biases of 
% the trained perceptron.
function [W, b] = train(P, T, f, maxIterations, seed)
    #rng(seed);
    randn("seed",seed)
  rand("seed", seed)
	W = rand(size(T, 2), size(P, 2));
	b = rand(size(W, 1), 1);
	i = 1;
	for x = 1:maxIterations
		p = P(i, :)';
		t = T(i, :)';
		a = perceptron(p, f, W, b);
		e = t - a;
		W = W + e * p';
		b = b + e;
		i = i + 1;
		if i > size(P, 1)
			i = 1;
		end
	end
end
