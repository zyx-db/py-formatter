def test(g=lambda x:next(filter(lambda x:x is not None,x),None),n=lambda x:None):
	return g([n(x:=10),[lambda:[lambda:'?',lambda:'hi2'][x > 8](),lambda:'hi'][x > 10]()])