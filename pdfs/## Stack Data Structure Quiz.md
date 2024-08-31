## Stack Data Structure Quiz

### Explanation

A **stack** is a fundamental linear data structure that operates on the **Last-In-First-Out (LIFO)** principle. Imagine a stack of plates: the last plate added is the first one you remove. Stacks are often used in programming for various tasks like function call management, expression evaluation, and undo/redo functionalities.

**Key Operations:**

* **Push:** Adds an element to the top of the stack.
* **Pop:** Removes and returns the top element from the stack.
* **Peek:** Returns the top element without removing it.
* **IsEmpty:** Checks if the stack is empty.
* **IsFull:** Checks if the stack is full (for fixed-size stacks).

**Applications:**

* **Recursion:** Stack helps manage function calls in recursion.
* **Expression Evaluation:** Stacks are used to convert infix expressions to postfix and evaluate postfix expressions.
* **Depth-First Search (DFS):** Stack is used to implement DFS for traversing graphs.
* **Undo/Redo Operations:**  Stacks store actions for undo and redo functionalities in text editors and other applications.
* **Browser History:** Browsers use stacks to store the visited web pages.
* **Function Calls:** Stacks are used to manage the order of function calls in a program.

**Implementations:**

Stacks can be implemented using various data structures, primarily:

* **Arrays:** Fixed-size stacks with a pointer to the top element.
* **Linked Lists:** Dynamically sized stacks using nodes connected in a chain.

Understanding the LIFO principle and the core operations of stacks is crucial for utilizing them effectively in your programs. 

### Multiple-Choice Questions

**Easy Level**

1. **Question 1**: What is the primary principle followed by a stack data structure?
   - (A) First-In-First-Out (FIFO)
   - (B) Last-In-First-Out (LIFO)
   - (C) Random Access
   - (D) Priority Based
   - **Correct Answer**: (B) 

2. **Question 2**: Which operation adds an element to the top of a stack?
   - (A) Pop
   - (B) Peek
   - (C) Push
   - (D) IsEmpty
   - **Correct Answer**: (C)

3. **Question 3**: Which operation retrieves the top element of a stack without removing it?
   - (A) Pop
   - (B) Peek
   - (C) Push
   - (D) IsFull
   - **Correct Answer**: (B)

4. **Question 4**: What does "IsEmpty" operation check in a stack?
   - (A) If the stack is full
   - (B) If the stack is empty
   - (C) The value of the top element
   - (D) The size of the stack
   - **Correct Answer**: (B)

5. **Question 5**: How is a stack often visualized?
   - (A) A queue of people
   - (B) A pile of books
   - (C) A stack of plates
   - (D) A tree structure
   - **Correct Answer**: (C)

6. **Question 6**: Which data structure can be used to implement a stack?
   - (A) Arrays
   - (B) Linked Lists
   - (C) Both A and B
   - (D) None of the above
   - **Correct Answer**: (C)

7. **Question 7**: What is the primary advantage of using a linked list for stack implementation?
   - (A) Fixed size
   - (B) Faster access to elements
   - (C) Dynamic size
   - (D) Easier to implement
   - **Correct Answer**: (C)

8. **Question 8**: Which of the following is NOT a common application of stacks?
   - (A) Function call management
   - (B) Expression evaluation
   - (C) Sorting algorithms
   - (D) Undo/redo functionalities
   - **Correct Answer**: (C)

9. **Question 9**: What is the order of operations for a stack?
   - (A) FIFO (First In First Out)
   - (B) LIFO (Last In First Out)
   - (C) FILO (First In Last Out)
   - (D) None of the above
   - **Correct Answer**: (B)

10. **Question 10**: Which of the following operations can change the size of a stack?
   - (A) Push
   - (B) Pop
   - (C) Peek
   - (D) Both A and B
   - **Correct Answer**: (D)

**Medium Level**

1. **Question 11**: Which of the following scenarios best illustrates the LIFO principle of a stack?
   - (A) People waiting in line at a movie theater
   - (B) A deck of cards being shuffled
   - (C) Books on a shelf arranged from tallest to shortest
   - (D) A pile of dishes being washed, with the dirtiest ones on top
   - **Correct Answer**: (D)

2. **Question 12**: In a stack implemented using an array, what happens when you try to push an element onto a full stack?
   - (A) The element is added to the top
   - (B) The element is added to the bottom
   - (C) An overflow error occurs
   - (D) The element is discarded
   - **Correct Answer**: (C)

3. **Question 13**:  What is the main advantage of using a stack in recursion?
   - (A) It allows for faster execution of recursive functions
   - (B) It provides a way to keep track of function calls and return addresses
   - (C) It helps in sorting data efficiently
   - (D) It ensures that recursive functions always terminate
   - **Correct Answer**: (B)

4. **Question 14**: Which of the following is a common technique for evaluating postfix expressions using a stack?
   - (A) Convert postfix to infix, then evaluate
   - (B) Use a stack to store operators and operands, processing them in reverse order
   - (C) Store operands in a stack and operators in a queue
   - (D) Use recursion to evaluate each operand individually
   - **Correct Answer**: (B)

5. **Question 15**:  How is a stack used in depth-first search (DFS) algorithms?
   - (A) To store visited nodes for efficient traversal
   - (B) To maintain the order of nodes explored
   - (C) To backtrack when a dead end is reached
   - (D) All of the above
   - **Correct Answer**: (D)

**Hard Level**

1. **Question 16**:  Suppose you have a stack of integers with the following elements: 1, 2, 3, 4 (with 4 at the top). What is the resulting stack after performing the following sequence of operations: Push(5), Pop(), Peek(), Push(6)?
   - (A) 1, 2, 3, 5, 6
   - (B) 1, 2, 3, 5
   - (C) 1, 2, 3, 6
   - (D) 1, 2, 3, 4, 6
   - **Correct Answer**: (C)

2. **Question 17**:  In a stack implemented using a linked list, how can you efficiently check if the stack is empty without traversing the entire list?
   - (A) Use a counter to track the number of elements
   - (B) Check if the head pointer of the linked list is null
   - (C) Iterate through the list and count the elements
   - (D) There is no efficient way to check for an empty stack in a linked list implementation
   - **Correct Answer**: (B)

3. **Question 18**:  What is the time complexity of the Push, Pop, and Peek operations in a stack implemented using an array?
   - (A) O(1) for all operations
   - (B) O(n) for Push and Pop, O(1) for Peek
   - (C) O(n) for Peek, O(1) for Push and Pop
   - (D) O(n) for all operations
   - **Correct Answer**: (A)

4. **Question 19**:  Explain how a stack can be used to implement an undo/redo functionality in a text editor.
   - **Correct Answer**: The text editor can use two stacks, one for undo actions and one for redo actions. When the user performs an action, like typing or deleting text, the action is pushed onto the undo stack. When the user wants to undo the action, the top element is popped from the undo stack and applied in reverse. The redo stack is used to store actions that have been undone, allowing the user to redo them later.

5. **Question 20**:  Imagine you have two stacks, A and B. You want to move elements from stack A to stack B in a way that maintains the order of elements in stack A. How can you achieve this efficiently?
   - **Correct Answer**: Use a loop to iterate through the elements in stack A. For each element, pop it from stack A and push it onto stack B. This will reverse the order of elements in stack B, but you can then pop the elements from stack B and push them back onto stack A to restore the original order.

### Descriptive Questions

1. **Question 1**: Describe the difference between a stack and a queue, and provide real-world examples of each.
2. **Question 2**: Explain how a stack is used to convert an infix expression to postfix expression. Provide an example.
3. **Question 3**: Discuss the advantages and disadvantages of using an array versus a linked list to implement a stack.
4. **Question 4**: How can you reverse a stack without using any additional data structures? Explain the logic and provide an algorithm.
5. **Question 5**: Describe how stacks are used in the implementation of browser history. 
6. **Question 6**:  Explain how a stack is utilized to manage function calls during the execution of a recursive function. Provide an example.
7. **Question 7**:  Describe the concept of a "monotonic stack" and its applications. Provide an example.
8. **Question 8**: Explain the difference between "stack overflow" and "stack underflow" errors, and provide scenarios for each.
9. **Question 9**:  How can you implement two stacks using a single array efficiently? Explain the data structure and the algorithms for push, pop, and peek operations.
10. **Question 10**: Discuss the limitations and potential performance issues associated with stack data structures, and provide scenarios where other data structures might be more suitable.

### Coding Practice Questions

1. **Implement a stack using an array:** Write a program in your preferred language to implement a stack data structure using an array. Include methods for push, pop, peek, isEmpty, and isFull operations.
2. **Implement a stack using a linked list:** Write a program to implement a stack using a linked list. Include methods for push, pop, peek, isEmpty, and isFull operations.
3. **Convert infix expression to postfix:** Write a program to convert an infix expression to postfix expression using a stack. Handle operator precedence and parentheses. 
4. **Evaluate a postfix expression:** Write a program to evaluate a postfix expression using a stack. Handle arithmetic operations like addition, subtraction, multiplication, and division.
5. **Implement depth-first search (DFS) using a stack:** Write a program to perform depth-first search on a graph using a stack.
6. **Implement an undo/redo functionality for a simple text editor:**  Write a program to implement undo/redo functionalities in a text editor using stacks. 
