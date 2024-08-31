## Linked List Quiz

### Multiple-Choice Questions

**Easy Level**

1. **Question 1**: What is the primary advantage of using a linked list over an array?
   - (A) Faster random access to elements
   - (B) More efficient insertion and deletion operations
   - (C) Requires less memory allocation
   - (D) Easier to implement
   - **Correct Answer**: (B)

2. **Question 2**: Which of the following is NOT a type of linked list?
   - (A) Singly Linked List
   - (B) Doubly Linked List
   - (C) Circular Linked List
   - (D) Linear Linked List
   - **Correct Answer**: (D)

3. **Question 3**: What does each node in a linked list typically contain?
   - (A) Only data
   - (B) Only a pointer to the next node
   - (C) Both data and a pointer to the next node
   - (D) Data, a pointer to the next node, and a pointer to the previous node
   - **Correct Answer**: (C)

4. **Question 4**: In a singly linked list, how do you access the last node?
   - (A) By directly accessing its memory address
   - (B) By traversing the list from the head until the next pointer is NULL
   - (C) By using a specific function that returns the last node
   - (D) By using a random access index
   - **Correct Answer**: (B)

5. **Question 5**: What is a header linked list?
   - (A) A linked list with a special node at the beginning called the header
   - (B) A linked list where the head and tail nodes are connected
   - (C) A linked list where each node points to both the previous and next node
   - (D) A linked list that stores data in a circular manner
   - **Correct Answer**: (A)

6. **Question 6**: What is the main difference between a singly linked list and a doubly linked list?
   - (A) Doubly linked lists have a special node at the beginning called the header
   - (B) Singly linked lists allow for faster insertion operations
   - (C) Doubly linked lists allow traversal in both directions
   - (D) Doubly linked lists are more efficient for storing large amounts of data
   - **Correct Answer**: (C)

7. **Question 7**: How many nodes are there in a circular linked list?
   - (A) Only one
   - (B) At least two
   - (C) Any number
   - (D) A fixed number depending on the implementation
   - **Correct Answer**: (C)

8. **Question 8**: Which operation is generally more efficient in a linked list compared to an array?
   - (A) Accessing a specific element
   - (B) Inserting an element at the beginning
   - (C) Deleting an element from the middle
   - (D) Sorting the elements
   - **Correct Answer**: (C)

9. **Question 9**: What is the purpose of a pointer in a linked list?
   - (A) To store the data of the node
   - (B) To connect nodes together
   - (C) To keep track of the size of the list
   - (D) To perform operations on the linked list
   - **Correct Answer**: (B)

10. **Question 10**: Which data structure can be implemented using a linked list?
   - (A) Stack
   - (B) Queue
   - (C) Deque
   - (D) All of the above
   - **Correct Answer**: (D)


**Medium Level**

1. **Question 11**: Which of the following is a common application of linked lists?
   - (A) Implementing a hash table
   - (B) Representing a graph
   - (C) Dynamic memory allocation
   - (D) All of the above
   - **Correct Answer**: (D)

2. **Question 12**: How would you find the middle node of a singly linked list?
   - (A) By traversing the list and counting the nodes
   - (B) By using a recursive function that traverses the list
   - (C) By using two pointers, one moving twice as fast as the other
   - (D) By accessing the middle node directly
   - **Correct Answer**: (C)

3. **Question 13**: How would you reverse a singly linked list?
   - (A) By swapping the data of the first and last nodes
   - (B) By iterating through the list and reversing the pointers
   - (C) By using a recursive function that reverses the list
   - (D) By using a stack to store the nodes in reverse order
   - **Correct Answer**: (B)

4. **Question 14**: What is the time complexity of deleting a node at a given position in a singly linked list?
   - (A) O(1)
   - (B) O(n)
   - (C) O(log n)
   - (D) O(n^2)
   - **Correct Answer**: (B)

5. **Question 15**: What is a common way to detect a loop in a linked list?
   - (A) By using a recursive function
   - (B) By using a hash table to store the nodes encountered
   - (C) By using two pointers, one moving twice as fast as the other
   - (D) By comparing the head and tail pointers
   - **Correct Answer**: (C)

6. **Question 16**: What is the difference between a circular linked list and a doubly linked list?
   - (A) Circular linked lists allow for faster insertion operations
   - (B) Doubly linked lists have a special node at the beginning called the header
   - (C) Circular linked lists have the last node pointing to the first node
   - (D) Doubly linked lists are more efficient for storing large amounts of data
   - **Correct Answer**: (C)

7. **Question 17**: What is the time complexity of searching for a specific element in a singly linked list?
   - (A) O(1)
   - (B) O(n)
   - (C) O(log n)
   - (D) O(n^2)
   - **Correct Answer**: (B)

8. **Question 18**: What is the advantage of using a circular doubly linked list compared to a regular doubly linked list?
   - (A) Circular doubly linked lists are more efficient for insertion operations
   - (B) Circular doubly linked lists allow for faster traversal of the list
   - (C) Circular doubly linked lists can be used to implement data structures like queues and stacks
   - (D) Circular doubly linked lists are easier to implement
   - **Correct Answer**: (C)

9. **Question 19**:  How would you find the intersection point of two linked lists?
   - (A) By comparing the data of each node
   - (B) By traversing both lists simultaneously and comparing the pointers
   - (C) By using a hash table to store the nodes of one list and then searching for them in the other
   - (D) By using a recursive function that traverses both lists
   - **Correct Answer**: (C)

10. **Question 20**: How would you modify the contents of a linked list while preserving its structure?
   - (A) By deleting and inserting new nodes
   - (B) By changing the data of existing nodes
   - (C) By reversing the pointers of the nodes
   - (D) By creating a new linked list with the modified contents
   - **Correct Answer**: (B)


**Hard Level**

1. **Question 21**: How would you implement a queue using a circular linked list?
   - (A) By using the head pointer as the front and the tail pointer as the rear
   - (B) By using the head pointer as the rear and the tail pointer as the front
   - (C) By using a separate pointer for the front and rear
   - (D) By using a recursive function that implements the queue operations
   - **Correct Answer**: (A)

2. **Question 22**: How would you implement a stack using a singly linked list?
   - (A) By using the head pointer as the top of the stack
   - (B) By using the tail pointer as the top of the stack
   - (C) By using a separate pointer for the top of the stack
   - (D) By using a recursive function that implements the stack operations
   - **Correct Answer**: (A)

3. **Question 23**: What is the time complexity of merging two sorted linked lists into a single sorted linked list?
   - (A) O(1)
   - (B) O(n)
   - (C) O(log n)
   - (D) O(n^2)
   - **Correct Answer**: (B)

4. **Question 24**: How would you sort a singly linked list using quicksort?
   - (A) By using a recursive function that partitions the list and sorts the sublists
   - (B) By using a loop that iterates through the list and compares adjacent nodes
   - (C) By using a hash table to store the nodes and then sorting them
   - (D) By converting the linked list to an array and then using quicksort on the array
   - **Correct Answer**: (A)

5. **Question 25**: How would you copy a linked list that has both next and arbitrary pointers?
   - (A) By using a recursive function that copies the nodes and pointers
   - (B) By creating a new linked list with the same data and pointers
   - (C) By using a hash table to store the nodes and pointers of the original list and then using them to create a new list
   - (D) By using a two-pass algorithm that first copies the nodes and then copies the arbitrary pointers
   - **Correct Answer**: (D)

6. **Question 26**:  How would you implement a priority queue using a doubly linked list?
   - (A) By using the head pointer as the highest priority and the tail pointer as the lowest priority
   - (B) By maintaining a sorted order of nodes based on their priority
   - (C) By using a separate pointer for each priority level
   - (D) By using a recursive function that implements the priority queue operations
   - **Correct Answer**: (B)

7. **Question 27**: How would you reverse a doubly linked list in groups of a given size?
   - (A) By using a loop that iterates through the list and reverses the pointers of each group
   - (B) By using a recursive function that reverses the pointers of each group
   - (C) By using a stack to store the nodes of each group in reverse order
   - (D) By using a hash table to store the nodes of each group and then reversing them
   - **Correct Answer**: (A)

8. **Question 28**: How would you reverse a stack without using extra space in O(n) time?
   - (A) By using a loop that iterates through the stack and reverses the order of the elements
   - (B) By using a recursive function that reverses the stack
   - (C) By using a temporary variable to swap the elements
   - (D) By using a doubly linked list to implement the stack and reversing the pointers
   - **Correct Answer**: (D)

9. **Question 29**: How would you represent a disjoint set data structure using a linked list?
   - (A) By using a separate linked list for each set
   - (B) By using a single linked list and storing the set information in the nodes
   - (C) By using a circular linked list and storing the set information in the nodes
   - (D) By using a doubly linked list and storing the set information in the nodes
   - **Correct Answer**: (A)

10. **Question 30**: How would you search for a sublist within another linked list?
   - (A) By comparing the data of each node in the sublist with the corresponding node in the main list
   - (B) By using a loop that iterates through the main list and checks for the sublist
   - (C) By using a recursive function that searches for the sublist
   - (D) By using a hash table to store the nodes of the sublist and then searching for them in the main list
   - **Correct Answer**: (B)

### Descriptive Questions

1. **Question 1**: Explain the advantages and disadvantages of using a linked list compared to an array. Provide specific examples to illustrate your points.
2. **Question 2**: Describe the process of inserting a new node at the beginning, middle, and end of a singly linked list. Illustrate each process with a diagram.
3. **Question 3**: Explain the difference between a singly linked list and a doubly linked list. When would you choose to use one over the other?
4. **Question 4**: How would you search for a specific element in a singly linked list? What is the time complexity of this operation?
5. **Question 5**: Describe the process of deleting a node from a singly linked list. Consider deleting a node from the beginning, middle, and end of the list.
6. **Question 6**: Explain how to reverse a singly linked list. Provide a step-by-step algorithm and illustrate it with an example.
7. **Question 7**: What is a circular linked list? How does it differ from a regular linked list? Describe its applications.
8. **Question 8**: Explain how you would detect a loop in a linked list using the "fast and slow pointer" method.
9. **Question 9**: Describe how you would implement a stack using a linked list. Provide a code snippet for the push and pop operations.
10. **Question 10**: Explain how you would implement a queue using a linked list. Provide a code snippet for the enqueue and dequeue operations.
11. **Question 11**: Describe how you would merge two sorted linked lists into a single sorted linked list. Provide a code snippet for the merge operation.
12. **Question 12**: Explain the concept of a priority queue and how you would implement it using a doubly linked list. Provide a code snippet for the insert and extract min operations.

### Coding Practice Questions

1. **Question 1**: Write a function to find the length of a singly linked list.
2. **Question 2**: Write a function to reverse a singly linked list.
3. **Question 3**: Write a function to find the middle node of a singly linked list.
4. **Question 4**: Write a function to check if a linked list is circular.
5. **Question 5**: Write a function to delete a node from a singly linked list at a given position.
6. **Question 6**: Write a function to merge two sorted linked lists into a single sorted linked list.
7. **Question 7**: Write a function to implement a stack using a linked list.
8. **Question 8**: Write a function to implement a queue using a linked list.
9. **Question 9**: Write a function to detect a loop in a linked list using the "fast and slow pointer" method.
10. **Question 10**: Write a function to copy a linked list that has both next and arbitrary pointers.

These coding questions can be solved using any programming language that supports linked lists. The solutions should be tested thoroughly with different test cases to ensure their correctness.

By completing these quiz questions and coding practice questions, learners will gain a solid understanding of linked lists, their applications, and their advantages and disadvantages compared to other data structures. 
