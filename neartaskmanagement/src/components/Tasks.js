import React, { useState } from 'react'
import Task from './Task'

export default function Tasks({ tasks, onDelete, onToggle }) {
    return (
        <>
            {tasks.map((task, index) => (
                <Task 
                key={index} 
                task={task} 
                onDelete={onDelete} 
                onToggle={onToggle} 
                />
            ))}
        </>
    )
}

