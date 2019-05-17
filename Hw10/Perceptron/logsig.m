% logsig(n)
%
% a = logsig returns the evauation of the log-sigmoid activation function.
function a = logsig(n)
	a = 1 / (1 + exp(-n));
end