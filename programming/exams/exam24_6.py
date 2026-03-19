class Node:
	def __init__(self, value):
		self.value = value
		self.next = None

def chain(arr):
	if len(arr) > 0:
		node = Node(arr[0])
		node.next = chain(arr[1:])
		return node
	else:
		return None

chain(['a', 'b', 'c'])
