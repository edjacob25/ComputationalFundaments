% accuracy(P, T, f, W, b)
%
% accuracy = accuracy calculates the accuracy of a trained perceptron on a set of examples.
function accuracy = accuracy(P, T, f, W, b)
    accuracy = 0;
	for i = 1:size(P, 1)
		p = P(i, :)';
		t = T(i, :)';
		a = perceptron(p, f, W, b);		
		if isequal(t, a)
			accuracy = accuracy + 1;
        end
    end
    accuracy = accuracy / size(P, 1);
end
