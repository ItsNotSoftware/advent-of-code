package lib

type Queue[T any] struct {
	elements []T
}

func (q *Queue[T]) Push(value T) {
	q.elements = append(q.elements, value)
}

func (q *Queue[T]) Pop() (T, bool) {
	if len(q.elements) == 0 {
		var zeroValue T
		return zeroValue, false
	}
	dequeued := q.elements[0]
	q.elements = q.elements[1:]
	return dequeued, true
}

func (q *Queue[T]) Peek() (T, bool) {
	if len(q.elements) == 0 {
		var zeroValue T
		return zeroValue, false
	}
	return q.elements[0], true
}

func (q *Queue[T]) IsEmpty() bool {
	return len(q.elements) == 0
}

func (q *Queue[T]) Size() int {
	return len(q.elements)
}
