import { EyeInvisibleOutlined, EyeTwoTone } from "@ant-design/icons";
import { Form, Input, Modal } from "antd";

interface Props {
  shouldShowModal: boolean;
  onSubmit: (token: string) => void;
  onCancel: () => void;
}

export function AuthModal({ shouldShowModal, onSubmit, onCancel }: Props) {
  const [form] = Form.useForm();

  const onFormSubmit = () => {
    form.validateFields().then((values) => {
      onSubmit(values.token);
    });
  };

  return (
    <Modal
      title="Provide Github Authentication Token"
      centered
      okText="Save"
      cancelText="Cancel"
      open={shouldShowModal}
      onOk={onFormSubmit}
      onCancel={onCancel}
    >
      <Form
        form={form}
        name="auth_form"
        initialValues={{
          token: "",
        }}
      >
        <Form.Item
          name="token"
          label="Token"
          rules={[
            {
              required: true,
              message: "Please provide your Github token",
            },
          ]}
        >
          <Input.Password
            placeholder="Github Token"
            iconRender={(visible) => {
              return visible ? <EyeTwoTone /> : <EyeInvisibleOutlined />;
            }}
          />
        </Form.Item>
      </Form>
    </Modal>
  );
}
