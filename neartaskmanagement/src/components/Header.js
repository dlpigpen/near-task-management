import React from 'react'
import PropTypes from 'prop-types'
import Button from './button'
import { useLocation } from 'react-router-dom'

const Header = ({ title, onAdd, showAdd, login }) => {
  const location = useLocation()

  return (
    <header className='header'>
      <h1>{title}</h1>
      {location.pathname === '/' && (window.walletConnection.isSignedIn()) ? (
        <Button
          color={showAdd ? 'red' : 'green'}
          text={showAdd ? 'Close' : 'Add'}
          onClick={onAdd}
        />
      ) :
        (<Button text="Connect to Near Wallet" onClick={login} />)
      }
    </header>
  )
}

Header.defaultProps = {
  title: 'Task Tracker'
}

Header.propTypes = {
  title: PropTypes.string.isRequired,
}
export default Header
