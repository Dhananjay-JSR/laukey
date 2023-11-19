import {useEffect, useState} from 'react'
import axios from "axios";

function ISODateString(d: Date) {
    function pad(n: number) {
        return n < 10 ? '0' + n : n
    }

    return d.getUTCFullYear() + '-'
        + pad(d.getUTCMonth() + 1) + '-'
        + pad(d.getUTCDate()) + 'T'
        + pad(d.getUTCHours()) + ':'
        + pad(d.getUTCMinutes()) + ':'
        + pad(d.getUTCSeconds()) + 'Z'
}


function App() {
    const [firstTime, setFirstTime] = useState(true)
    const [userNameInput, setUserNameInput] = useState("")
    const [userPasswordInput, setUserPasswordInput] = useState("")
    const [screenType, setScreenType] = useState<"DASHBOARD" | "LOGIN">("LOGIN")
    const [loginScreenType, setLoginScreenType] = useState<"CRED_ENTER" | "SET_PASS">("CRED_ENTER")
    const [RootPasword, setRootPassword] = useState("")
    useEffect(() => {
        axios.get("http://localhost:3000/state").then((d) => {
            setFirstTime(() => d.data.newSetup)
        })
    }, [])


    return (
        <>
            <div className={"min-h-screen bg-neutral-900 w-full flex "}>
                {screenType == "LOGIN" && <div className={"w-96 h-64   rounded mx-auto bg-black  my-auto text-white"}>
                    <div className={"text-white text-center mt-3 font-semibold"}>Welcome to Laukey Dashboard</div>
                    {loginScreenType != "CRED_ENTER" && firstTime &&
                        <div className={"text-xs mt-2 text-center"}>System Will Go Through First Time Setup</div>}

                    {loginScreenType == "CRED_ENTER" ? <div className={"mx-auto mt-8 w-fit"}>
                        <div className={"mt-3 mx-auto"}>
                            <span className={"mr-2"}>UserName</span><input value={userNameInput} onChange={(e) => {
                            let UserName = e.currentTarget.value
                            setUserNameInput(() => UserName)
                        }} className={"bg-neutral-700 px-2"}/>
                        </div>
                        <div className={"mt-3 mx-auto flex justify-between"}>
                            <span className={"mr-2"}>Password</span><input value={userPasswordInput} onChange={(e) => {
                            let UserPass = e.currentTarget.value
                            setUserPasswordInput(() => UserPass)
                        }} className={"bg-neutral-700 px-2"}/>
                        </div>
                        {firstTime &&
                            <h2 className={"text-xs mt-4  text-center"}>Login with Default credential admin /
                                admin</h2>}
                    </div> : <>
                        <div className={"mx-auto mt-8 w-fit"}>
                            <div className={"mt-3 mx-auto"}>
                                <div className={"text-center  mb-3"}>Enter New Admin Password</div>
                                <span className={"mr-2"}>Root Password</span><input value={RootPasword}
                                                                                    onChange={(e) => {
                                                                                        let RootPasswordInput = e.currentTarget.value
                                                                                        setRootPassword(() => RootPasswordInput)
                                                                                    }}
                                                                                    className={"bg-neutral-700 px-2"}/>
                            </div>
                        </div>
                    </>}
                    <button onClick={() => {

                        if (firstTime) {
                            if (userNameInput == "admin" || userPasswordInput == "admin") {

                                if (loginScreenType == "CRED_ENTER") {

                                    setLoginScreenType("SET_PASS")
                                    setFirstTime(false)
                                }

                                if (loginScreenType == "SET_PASS") {
                                    // alert(RootPasword)

                                    axios.post("http://localhost:3000/adminpass", {
                                        userName: userNameInput,
                                        passWord: RootPasword
                                    }).then((d) => {
                                        if (d.data.statusCode == 200) {
                                            alert("Request Accepted Please Use New Password to Login")
                                            window.location.reload()
                                        } else {
                                            alert("Unknown Error Occurred")
                                        }
                                    })

                                }

                                // setScreenType(prev=>"DASHBOARD")
                            }
                            // axios.post("http://localhost:3000/adminpass")
                        } else {
                            axios.post("http://localhost:3000/login", {
                                userName: userNameInput,
                                passWord: userPasswordInput
                            }).then((d) => {
                                if (d.data.login == true) {
                                    setScreenType("DASHBOARD")
                                } else {
                                    alert("Wrong Credential")
                                }
                            })
                        }
                    }} className={"bg-amber-700 px-3 py-1 rounded  block mx-auto mt-3"}>
                        Login
                    </button>

                </div>}
                {screenType == "DASHBOARD" && <QuerryDashboard/>}

            </div>
        </>
    )
}


function QuerryDashboard() {
    const [selectEdFilters, setSelectedFilter] =
        useState<string[]>([])
    const [selectedOption, setSelectedOption] = useState("Select Querry")


    // Query Fields State
    const [LevelQuerry, setLevelQuerry] = useState("")
    const [MessageQuerry, setMessageQuerry] = useState("")
    const [ResourceIDQuerry, setResourceIDQuerry] = useState("")
    const [FromTimeStamp, setFromTimeStamp] = useState("")
    const [ToTimeStamp, setToTimeStamp] = useState("")
    const [TraceIDQuerry, setTraceIDQuerry] = useState("")
    const [SpanQuerry, setSpanQuerry] = useState("")
    const [CommitQuerry, setCommitQuerry] = useState("")
    const [ParentResourceQuerry, setParentResourceQuerry] = useState("")
    // Request Handler
    const [isLoading, setIsLoading] = useState(false)
    const [ResponseData, setResponseData] = useState<any>(null)


    useEffect(() => {
        if (selectedOption == "Select Querry") {
            return;
        }
        setSelectedOption(() => ["Level", "Message", "ResourceId", "Timestamp", "TraceId", "SpanId", "Commit", "ParentResourceId"].filter(e => !selectEdFilters.includes(e))[0] ?? "Select Querry")
    }, [selectEdFilters]);

    return <div className={"w-full"}>
        <div className={"bg-neutral-900 h-[450px] max-h-full w-[650px] mx-auto mt-8 relative"}>
            <div className={"text-white text-center underline"}>Querry Interface</div>
            <div className={"mx-auto w-fit mt-5"}>
                <div className={"mx-auto flex justify-center"}>
                    <select value={selectedOption} onChange={(e) => {
                        let SelectedOption = e.currentTarget.value
                        setSelectedOption(() => SelectedOption)
                    }}>
                        <option disabled>Select Querry</option>
                        {["Level", "Message", "ResourceId", "Timestamp", "TraceId", "SpanId", "Commit", "ParentResourceId"].filter(e => !selectEdFilters.includes(e)).map((e) =>
                            <option value={e}>{e}</option>)}
                    </select>
                    <button className={"ml-3 bg-amber-300  px-3 "} onClick={() => {
                        if (selectedOption != "Select Querry") {

                            setSelectedFilter(prev => {

                                return [...prev, selectedOption]
                            })
                        }
                    }}>Add
                    </button>

                </div>

                <div className={"mt-4"}>
                    {selectEdFilters.includes("Level") && <>
                        <div className={"flex gap-3"}>

                            <div className={"text-white w-36"}>Level</div>
                            <input value={LevelQuerry} onChange={(e) => {
                                let Value = e.currentTarget.value
                                setLevelQuerry(() => Value)
                            }}/>
                            <button className={"bg-white px-2"} onClick={() => {
                                setSelectedFilter(prev => prev.filter((e) => e != "Level"))
                                setLevelQuerry("")
                            }}>Remove Filter
                            </button>
                        </div>
                    </>}

                    <div className={"flex flex-col "}>
                        {selectEdFilters.includes("Message") && <>
                            <div className={"flex gap-3 mt-5"}>

                                <div className={"text-white w-36"}>Message</div>
                                <input value={MessageQuerry} onChange={(e) => {
                                    let Value = e.currentTarget.value
                                    setMessageQuerry(() => Value)
                                }}/>
                                <button className={"bg-white px-2"} onClick={() => {
                                    setSelectedFilter(prev => prev.filter((e) => e != "Message"))
                                    setMessageQuerry("")
                                }}>Remove Filter
                                </button>
                            </div>
                        </>}
                        {selectEdFilters.includes("ResourceId") && <>
                            <div className={"flex gap-3 mt-5"}>

                                <div className={"text-white w-36"}>ResourceId</div>
                                <input value={ResourceIDQuerry} onChange={(e) => {
                                    let Value = e.currentTarget.value
                                    setResourceIDQuerry(() => Value)
                                }}/>
                                <button className={"bg-white px-2"} onClick={() => {
                                    setSelectedFilter(prev => prev.filter((e) => e != "ResourceId"))
                                    setResourceIDQuerry("")
                                }}>Remove Filter
                                </button>
                            </div>
                        </>}
                        {selectEdFilters.includes("Timestamp") && <>
                            <div className={"flex gap-3 mt-5 items-baseline"}>

                                <div className={"text-white w-36"}>Timestamp</div>
                                <div className={"flex gap-2 items-baseline"}>
                                    <input type={"datetime-local"} value={FromTimeStamp} onChange={(e) => {
                                        let Value = e.currentTarget.value
                                        setFromTimeStamp(() => Value)
                                    }}/>
                                    <input type={"datetime-local"} value={ToTimeStamp} onChange={(e) => {
                                        let Value = e.currentTarget.value
                                        setToTimeStamp(() => Value)
                                    }}/>
                                </div>
                                <button className={"bg-white px-2 items-baseline"} onClick={() => {
                                    setSelectedFilter(prev => prev.filter((e) => e != "ResourceId"))
                                    setToTimeStamp(() => "")
                                    setFromTimeStamp(() => "")
                                }}>Remove Filter
                                </button>
                            </div>
                        </>}


                        {selectEdFilters.includes("TraceId") && <>
                            <div className={"flex gap-3 mt-5"}>

                                <div className={"text-white w-36"}>TraceId</div>
                                <input value={TraceIDQuerry} onChange={(e) => {
                                    let Value = e.currentTarget.value
                                    setTraceIDQuerry(() => Value)
                                }}/>
                                <button className={"bg-white px-2"} onClick={() => {
                                    setSelectedFilter(prev => prev.filter((e) => e != "TraceId"))
                                    setTraceIDQuerry("")
                                }}>Remove Filter
                                </button>
                            </div>
                        </>}


                        {selectEdFilters.includes("SpanId") && <>
                            <div className={"flex gap-3 mt-5"}>

                                <div className={"text-white w-36"}>SpanId</div>
                                <input value={SpanQuerry} onChange={(e) => {
                                    let Value = e.currentTarget.value
                                    setSpanQuerry(() => Value)
                                }}/>
                                <button className={"bg-white px-2"} onClick={() => {
                                    setSelectedFilter(prev => prev.filter((e) => e != "SpanId"))
                                    setSpanQuerry("")
                                }}>Remove Filter
                                </button>
                            </div>
                        </>}


                        {selectEdFilters.includes("Commit") && <>
                            <div className={"flex gap-3 mt-5"}>

                                <div className={"text-white w-36"}>Commit</div>
                                <input value={CommitQuerry} onChange={(e) => {
                                    let value = e.currentTarget.value
                                    setCommitQuerry(() => value)
                                }}/>
                                <button className={"bg-white px-2"} onClick={() => {
                                    setSelectedFilter(prev => prev.filter((e) => e != "Commit"))
                                    setCommitQuerry("")
                                }}>Remove Filter
                                </button>
                            </div>
                        </>}


                        {selectEdFilters.includes("ParentResourceId") && <>
                            <div className={"flex gap-3 mt-5"}>

                                <div className={"text-white w-36"}>ParentResourceId</div>
                                <input value={ParentResourceQuerry} onChange={(e) => {
                                    let Value = e.currentTarget.value
                                    setParentResourceQuerry(() => Value)
                                }}/>
                                <button className={"bg-white px-2"} onClick={() => {
                                    setSelectedFilter(prev => prev.filter((e) => e != "ParentResourceId"))
                                    setParentResourceQuerry("")
                                }}>Remove Filter
                                </button>
                            </div>
                        </>}


                    </div>


                </div>
            </div>

            {selectEdFilters.length > 0 &&
                <button onClick={() => {
                    let BodyParams: {
                        level?: string,
                        message?: string,
                        resourceId?: string,
                        timestamp?: string,
                        traceId?: string,
                        spanId?: string,
                        commit?: string,
                        parentResourceId?: string
                    } = {};
                    if (LevelQuerry != "") {
                        BodyParams.level = LevelQuerry
                    }
                    if (MessageQuerry != "") {
                        BodyParams.message = MessageQuerry
                    }
                    if (ResourceIDQuerry != "") {
                        BodyParams.resourceId = ResourceIDQuerry
                    }

                    if (FromTimeStamp != "") {
                        if (ToTimeStamp != "") {
                            BodyParams.timestamp = `${ISODateString(new Date(FromTimeStamp))} TO ${ISODateString(new Date(ToTimeStamp))}`
                        } else {
                            BodyParams.timestamp = `${ISODateString(new Date(FromTimeStamp))} TO ${ISODateString(new Date())}`
                        }
                    }

                    if (TraceIDQuerry != "") {
                        BodyParams.traceId = TraceIDQuerry
                    }
                    if (SpanQuerry != "") {
                        BodyParams.spanId = SpanQuerry
                    }

                    if (CommitQuerry != "") {
                        BodyParams.commit = CommitQuerry
                    }
                    if (ParentResourceQuerry != "") {
                        BodyParams.parentResourceId = ParentResourceQuerry
                    }

                    console.log(BodyParams)
                    setIsLoading(true)
                    axios.get("http://localhost:3000/search", {
                        params: BodyParams
                    }).then((d) => {
                        setIsLoading(false)
                        // @ts-ignore
                        setResponseData(() => d)
                    })


                }} className={"absolute bottom-0 left-1/2 -translate-x-1/2 bg-red-100  px-2 rounded  mb-3"}>Run
                    Querry</button>}
        </div>
        {isLoading && <div className={"text-white animate-pulse text-center"}>Loading ...</div>}
        {ResponseData != null &&(ResponseData.data!=undefined) &&
            <div className={"text-teal-100 text-center"}>Logs Returned {ResponseData.data.length}</div>}
        {ResponseData != null && <pre className={"mx-auto ml-4"}>
<code className={"text-white"}>
    {JSON.stringify(ResponseData.data, null, 2)}

</code>

        </pre>
        }
    </div>
}

export default App
