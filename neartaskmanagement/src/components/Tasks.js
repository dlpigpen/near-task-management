import React, { useState } from 'react'
import Task from './Task'

export default function Tasks({ tasks, onDelete }) {
    return (
        <>
            {tasks.map((task) => (
                <Task key={task.id} task={task} onDelete={onDelete} />
            ))}
        </>
    )
}

