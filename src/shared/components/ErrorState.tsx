type ErrorStateProps = {
  title?: string;
  message: string;
};

export function ErrorState({ title = "Something went wrong", message }: ErrorStateProps) {
  return (
    <div className="state-message error-state" role="alert">
      <strong>{title}</strong>
      <span>{message}</span>
    </div>
  );
}
