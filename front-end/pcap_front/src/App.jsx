import { useState } from 'react'
import {BrowserRouter as Router, Routes, Route, Link} from 'react-router-dom'
import Login from './views/login/Login.jsx'
import './App.css'
import Capture from "./views/public/Capture.jsx";

function App() {


  return (
  <Router>
    <Routes>
      <Route path="/">
        <Route path={""} element={<Capture/>} />
        <Route path={"login"} element={<Login/>} />
      </Route>

    </Routes>
  </Router>
  )
}

export default App
