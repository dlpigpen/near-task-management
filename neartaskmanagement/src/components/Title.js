import React, { useEffect, useState } from 'react'
import { useLocation } from 'react-router-dom'

const Title = () => {

    const [totalPost, setTotalPost] = useState(0);
    useEffect(async () => {
        const result = window.contract.get_user_total_task({
            account_id: window.accountId
        })
        const total = await result;
        setTotalPost(total);

    });

    const location = useLocation()
    if (location.pathname !== '/') {
        return (
            <>
            </>
        )
    }

    return (
        <div className="title">
            <span>{parseInt(totalPost) > 0 ? `There are ${totalPost} tasks` : "No post"}</span>
        </div>
    )
}

export default Title
