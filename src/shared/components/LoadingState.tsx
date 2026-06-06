type LoadingStateProps = {
  label?: string;
};

export function LoadingState({ label = "Loading" }: LoadingStateProps) {
  return (
    <div className="state-message" role="status">
      <span>{label}</span>
    </div>
  );
}
