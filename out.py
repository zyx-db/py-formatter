def test_function(x,g=lambda x: next(filter(lambda x: x is not None, x), None),n=lambda x: None):
	return g([n(x:=10),n(x:=x + 5),n(print(x))])