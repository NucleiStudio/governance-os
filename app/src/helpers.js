/// parse a call from hexadecimal to a human understandable string for better UX
const parseCall = (api, call) => {
    let parsed = api.createType('Call', call);
    return `${parsed.section}.${parsed.method}(${parsed.args})`;
}

export {
    parseCall,
};