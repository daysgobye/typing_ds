

type ButtonProps = {
    selected?: boolean
    onClick: (e?: any) => void
    children: any
    oledInfo?: boolean
    tinyButtons?: boolean
    disabled?: boolean
    disabledOnClick?: () => void
    className?: string
    icon?: boolean

};

export default function Button(props: ButtonProps) {
    const returnClasses = () => {
        let classes = `btn mr-1`
        if (props.oledInfo) {
            classes += ` hover:btn-warning`
        } else {
            classes += ` `
        }
        return classes += returnButtonSizes()
    }
    const returnButtonSizes = () => {
        if (props.tinyButtons) {
            return ` btn-xs`
        } else {
            return ` btn-sm`
        }
    }
    const returnColor = () => {
        let color = ``
        if (props.oledInfo) {
            color += ` btn-warning`
        } else {
            return ` btn-primary`
        }
        return color
    }
    return (
        <button class={`${returnClasses()} ${props.selected ? `${returnColor()}` : ' btn-outline'} ${props.className} ${props.disabled ? 'btn-error cursor-not-allowed' : ''}`}
            onClick={(e: any) => {
                if (props.disabled !== undefined) {
                    if (props.disabled === false) {
                        props.onClick(e)
                    }
                } else {
                    props.onClick(e)
                }
            }}
        >
            {props.children}
        </button>
    );

}
