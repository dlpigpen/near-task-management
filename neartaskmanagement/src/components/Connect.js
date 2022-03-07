import React from 'react'

function Connect({ onLogout }) {
    if (window.walletConnection.isSignedIn()) {
        return (
            <button className="btn" style={{ margin: '0 auto', display: 'block' }} onClick={onLogout}>
                Sign out
            </button>
        )
    }
    return (
        <></>
    )
}

export default Connect
