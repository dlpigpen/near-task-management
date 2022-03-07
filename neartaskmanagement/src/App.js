import 'regenerator-runtime/runtime'
import React, { useState } from 'react'
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom'
import { login, logout } from './utils'

import './global.css'

import getConfig from './config'
import Header from './components/Header'
import Footer from './components/Footer'
import Tasks from './components/Tasks'
import AddTask from './components/AddTask'
import About from './components/About'
import Title from './components/Title';
import Connect from './components/Connect';
import Loading from './components/Loading';

const { networkId } = getConfig(process.env.NODE_ENV || 'development')


export default function App() {

  const [loading, setLoading] = useState(false)

  const [showAddTask, setShowAddTask] = useState(false)

  const [tasks, setTasks] = useState([])

  /// delete task
  const deleteTask = async (id) => {
    console.log(id)
    setLoading(true)
    const result = window.contract.delete_task_by_id(
      {
        task_id: id
      }
    );

    await result;
    setTasks(tasks.filter((task) => task.task_id !== id))
    setLoading(false)
  }


  // toggle reminder
  const toggleReminder = (id) => {
    setTasks(tasks.map((task) => task.task_id === id ? { ...task, reminder: !task.reminder } : task))
  }


  // add task
  const addTask = async (task) => {
    try {
      setLoading(true)
      setShowAddTask(false)

      const result = window.contract.create_task({
        text: task.text,
        day: task.day,
        reminder: task.reminder,
      });
    
      await result;
      setLoading(false)
      setShowAddTask(false)

      const newTask = { id, ...task }
      setTasks([...tasks, newTask])

    } catch (e) {
      toast.error(e.message);
    }
  }


  // The useEffect hook can be used to fire side-effects during render
  // Learn more: https://reactjs.org/docs/hooks-intro.html
  React.useEffect(
    () => {
      // in this case, we only care to query the contract when signed in
      if (window.walletConnection.isSignedIn()) {

        // window.contract is set by initContract in index.js
        window.contract.get_user_tasks({ account_id: window.accountId })
          .then(tasksFromContact => {
            console.log("task from contract:", tasksFromContact);
            setTasks(tasksFromContact)
          })
      } else {
        setTasks([])
      }
    },

    // The second argument to useEffect tells React when to re-run the effect
    // Use an empty array to specify "only run on first render"
    // This works because signing into NEAR Wallet reloads the page
    []
  )
  const notify = () => {
    toast("Will close after 15s", { autoClose: 5000 });
  }

  return (
    <Router>

      <div className="container">
        <Header
          onAdd={() => setShowAddTask(!showAddTask)}
          showAdd={showAddTask}
          login={login}

        />
        <Title />
        <Routes>
          <Route
            path='/'
            element={
              <>
                { loading && (<Loading />) }
                {showAddTask && <AddTask onAdd={addTask} />}
                {tasks.length > 0 ?
                  (<Tasks
                    tasks={tasks}
                    onDelete={deleteTask}
                    onToggle={toggleReminder} 
                    />) :
                  ('No task to show')
                }
              </>
            }
          />
          <Route path='/about' element={<About />} />
        </Routes>
        <Footer />


      </div>
      <Connect onLogout={logout} />
    </Router>
  )
}
