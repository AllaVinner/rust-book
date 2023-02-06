import numpy as np



def julia_fractal(c_re, c_im, re_min=-3, re_max=3, im_min=-3, im_max=3, image_width=800, image_height=800):
    rang_re = re_max-re_min
    rang_im = im_max-im_min

    scale_re = rang_re/image_width
    scale_im = rang_im/image_height

    image = np.zeros((image_width, image_height, 3), type=int)

    for pixel_w in range(image_width):
        for pixel_h in range(image_height):
            re = pixel_w * scale_re - rang_re/2
            im = -(pixel_h * scale_im - rang_im/2)

            i = 0
            factor = 20
            while (i<255/factor and re**2+im**2<=4):
                re = re**2-im**2 + c_re
                im = 2*re*im + c_im
                i += 1
            
            image[pixel_w, pixel_h, :] = np.array([0, int(i*factor), 0])

    return image


julia_fractal(-1.1, 0.4)
















